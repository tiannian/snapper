use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
};

use snapper_core::{ProfileType, SnapperFile};
use tokio::{
    fs::{self, File},
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
    CompilerInput, CompilerOutput, CompilerVersions, Error, Result,
};

pub struct Solc {
    snapper: SnapperFile,
    solc_path: PathBuf,
}

impl Solc {
    /// New a solc instance
    ///
    /// Notice: this function only can be call in `build.rs`
    pub async fn new<P: AsRef<Path>>(
        out_dir: P,
        upstream: Option<&str>,
        snapper: &str,
    ) -> Result<Self> {
        let versions = if let Some(upstream) = upstream {
            CompilerVersions::load_from(upstream).await?
        } else {
            CompilerVersions::load().await?
        };

        let snapper = utils::load_snapper_file(snapper)?;

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        let out_dir = out_dir.as_ref();

        let bin_dir = out_dir.join("bin");
        fs::create_dir_all(&bin_dir).await?;

        let solc_path = utils::solc_path(&bin_dir, &snapper.solidity.version)?;

        if !solc_path.exists() {
            versions
                .download(&snapper.solidity.version, &platform, &solc_path)
                .await?;
        }

        Ok(Self { snapper, solc_path })
    }

    pub async fn compile<P: AsRef<Path>>(
        &self,
        file: P,
        profile_type: &ProfileType,
        out_dir: P,
    ) -> Result<()> {
        let mut sources = HashMap::new();

        let file = file.as_ref();

        let filename = file
            .file_name()
            .ok_or(Error::FailedToParseFileName)?
            .to_string_lossy()
            .to_string();

        let sf = SourceFile {
            keccak256: None,
            urls: vec![file.to_string_lossy().to_string()],
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

        let profile = self.snapper.get_solidity_profile(profile_type);

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

        if !res.errors.is_empty() {
            println!("{:?}", res.errors);
            panic!("Solidity compile error");
        }

        if let Some(contracts) = res.contracts.ok_or(Error::NoContractOutput)?.get(&filename) {
            for (name, contract) in contracts.iter() {
                let out_dir = out_dir.as_ref();

                let contract_dir = out_dir.join(&filename).join(name);
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
}

#[cfg(test)]
mod tests {
    use snapper_core::ProfileType;
    use tokio::runtime::Runtime;

    use crate::Solc;

    #[test]
    fn test() {
        let out_dir = "../target";
        let snapper_file = "../cargo-snapper/assets/Snapper.toml";

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let solc = Solc::new(out_dir, None, snapper_file).await.unwrap();
            solc.compile(
                "contracts/Lock.sol",
                &ProfileType::Debug,
                "../target/solc-test/",
            )
            .await
            .unwrap();
        });
    }
}
