use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use snapper_core::{ProfileType, SnapperFile};
use tokio::{
    fs::{self, DirEntry, File},
    io::AsyncWriteExt,
    process::Command,
};

use crate::{
    input::{
        self, DebugInfo, Optimizer, OptimizerDetails, OutputSelection, RevertStrings,
        SettingsDebug, SourceFile, YulDetails,
    },
    utils,
    version::Platform,
    CompilerInput, CompilerOutput, CompilerVersions, Config, Error, Result,
};

pub struct Solc {
    out_dir: PathBuf,
    snapper: SnapperFile,
    solc_path: PathBuf,
}

impl Solc {
    /// New a solc instance
    ///
    /// Notice: this function only can be call in `build.rs`
    pub async fn new() -> Result<Self> {
        let out_dir = utils::default_snapper_outdir()?;

        Self::new_arguments(&out_dir, None, ".").await
    }

    pub async fn new_with_config(config: Config) -> Result<Self> {
        let out_dir = if let Some(out_dir) = config.out_dir {
            Path::new(&out_dir).to_path_buf()
        } else {
            utils::default_snapper_outdir()?
        };

        let snapper_file = if let Some(snapper) = &config.snapper_file {
            snapper
        } else {
            "."
        };

        Self::new_arguments(&out_dir, config.upstream.as_deref(), snapper_file).await
    }

    async fn new_arguments(out_dir: &Path, upstream: Option<&str>, snapper: &str) -> Result<Self> {
        let versions = if let Some(upstream) = upstream {
            CompilerVersions::load_from(upstream).await?
        } else {
            CompilerVersions::load().await?
        };

        let snapper = utils::load_snapper_file(snapper)?;

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        let out_dir = Path::new(out_dir);

        let bin_dir = out_dir.join("bin");
        fs::create_dir_all(&bin_dir).await?;

        let solc_path = utils::solc_path(&bin_dir, &snapper.solidity.version)?;

        if !solc_path.exists() {
            versions
                .download(&snapper.solidity.version, &platform, &solc_path)
                .await?;
        }

        Ok(Self {
            out_dir: out_dir.to_path_buf(),
            snapper,
            solc_path,
        })
    }

    async fn compile_one(&self, file: &DirEntry) -> Result<()> {
        let mut sources = HashMap::new();

        let path = file.path();
        let filename = file.file_name().to_string_lossy().to_string();

        let sf = SourceFile {
            keccak256: None,
            urls: vec![path.to_string_lossy().to_string()],
        };
        sources.insert(filename.clone(), sf);

        let mut output_selection = HashMap::new();

        let mut contract_output = HashMap::new();
        contract_output.insert(
            "*".to_string(),
            vec![
                OutputSelection::Abi,
                OutputSelection::EvmBytecode,
                OutputSelection::EvmGasEstimates,
                OutputSelection::EvmBytecodeSourceMap,
                OutputSelection::EvmDeployedBytecode,
            ],
        );

        output_selection.insert("*".to_string(), contract_output);

        let profile_s = env::var("PROFILE")?;
        let pt = ProfileType::from_str(&profile_s)?;
        let profile = self.snapper.get_solidity_profile(pt);

        let revert_strings = if profile.debug {
            RevertStrings::Debug
        } else {
            RevertStrings::Default
        };

        let optimizer = Optimizer {
            enabled: profile.optimizer.enable,
            runs: profile.optimizer.runs,
            details: OptimizerDetails {
                cse: profile.optimizer.cse,
                deduplicate: profile.optimizer.deduplicate,
                inliner: profile.optimizer.inliner,
                jumpdest_remover: profile.optimizer.remove_jumpdest,
                constant_optimizer: profile.optimizer.constant,
                order_literals: false,
                peephole: true,
                yul: true,
                yul_details: YulDetails {
                    stack_allocation: profile.optimizer.yul_stack,
                    optimizer_steps: None,
                },
            },
        };

        let input = CompilerInput {
            language: input::InputLanguage::Solidity,
            sources,
            settings: input::Settings {
                stop_after: None,
                remappings: vec![],
                optimizer,
                evm_version: self.snapper.solidity.evm_version.clone(),
                via_ir: self.snapper.solidity.via_ir,
                debug: SettingsDebug {
                    revert_strings,
                    debug_info: vec![DebugInfo::All],
                },
                metadata: None,
                libraries: self.snapper.library.clone(),
                output_selection,
                model_checker: None,
            },
        };

        let in_data = serde_json::to_string(&input)?;

        let mut command = Command::new(self.solc_path.clone())
            .arg("--standard-json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        command
            .stdin
            .take()
            .ok_or(Error::FailedToGetStdio)?
            .write_all(in_data.as_bytes())
            .await?;

        let output = command.wait_with_output().await?;

        let res: CompilerOutput = serde_json::from_slice(&output.stdout)?;

        println!("{:#?}", res);

        if !res.errors.is_empty() {
            println!("{:?}", res.errors);
            panic!("Solidity compile error");
        }

        if let Some(contracts) = res.contracts.ok_or(Error::NoContractOutput)?.get(&filename) {
            for (name, contract) in contracts.iter() {
                let contract_dir = self.out_dir.join("artifacts").join(&filename).join(name);
                fs::create_dir_all(&contract_dir).await?;

                let abi = &contract.abi;
                let bytecode = &contract.evm.bytecode.object;
                let opcodes = &contract.evm.bytecode.opcodes.trim();
                let sourcemap = &contract.evm.bytecode.source_map.trim();

                // TODO: Add estimate to info.

                let mut file = File::create(contract_dir.join(format!("{}.abi", name))).await?;
                file.write_all(serde_json::to_string(abi)?.as_bytes())
                    .await?;

                let mut file =
                    File::create(contract_dir.join(format!("{}.bytecode", name))).await?;
                file.write_all(bytecode).await?;

                let mut file = File::create(contract_dir.join(format!("{}.opcodes", name))).await?;
                file.write_all(opcodes.as_bytes()).await?;

                let mut file =
                    File::create(contract_dir.join(format!("{}.sourcemap", name))).await?;
                file.write_all(sourcemap.as_bytes()).await?;
            }
        }

        Ok(())
    }

    pub async fn compile(self, dir: &str) -> Result<()> {
        let mut all_files = fs::read_dir(dir).await?;

        while let Some(file) = all_files.next_entry().await? {
            if file.file_type().await?.is_file() {
                self.compile_one(&file).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use tokio::runtime::Runtime;

    use crate::{Config, Solc};

    #[test]
    fn test() {
        let config = Config {
            upstream: None,
            out_dir: None,
            snapper_file: Some("../cargo-snapper/assets/Snapper.toml".to_string()),
        };

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            env::set_var("PROFILE", "debug");
            env::set_var("CARGO_TARGET_DIR", "../target");

            let solc = Solc::new_with_config(config).await.unwrap();
            solc.compile("./contracts").await.unwrap();
        });
    }
}
