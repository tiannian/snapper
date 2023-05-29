use std::{collections::HashMap, env, path::PathBuf};

use snapper_core::{ProfileType, SnapperFile};
use tokio::{fs, io::AsyncWriteExt, process::Command};

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
        let versions = CompilerVersions::load().await?;
        let default_dir = env::var("OUT_DIR")?;
        let out_dir = PathBuf::from(default_dir);
        let snapper = utils::load_snapper_file(".")?;

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        let solc_path = versions
            .download(&snapper.solidity.version, &platform, &out_dir)
            .await?;

        Ok(Self {
            out_dir,
            snapper,
            solc_path,
        })
    }

    pub async fn new_with_config(config: Config) -> Result<Self> {
        let versions = if let Some(upstream) = config.upstream {
            CompilerVersions::load_from(&upstream).await?
        } else {
            CompilerVersions::load().await?
        };

        let out_dir = if let Some(out_dir) = config.out_dir {
            PathBuf::from(out_dir)
        } else {
            let default_dir = env::var("OUT_DIR")?;
            PathBuf::from(default_dir)
        };

        let snapper = if let Some(snapper) = config.snapper_file {
            utils::load_snapper_file(&snapper)?
        } else {
            utils::load_snapper_file(".")?
        };

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        let solc_path = versions
            .download(&snapper.solidity.version, &platform, &out_dir)
            .await?;

        Ok(Self {
            out_dir,
            snapper,
            solc_path,
        })
    }

    pub async fn compile(self) -> Result<()> {
        let mut all_files = fs::read_dir(".").await?;

        let mut sources = HashMap::new();

        while let Some(file) = all_files.next_entry().await? {
            if file.file_type().await?.is_file() {
                let path = file.path();
                let filename = file.file_name().to_string_lossy().to_string();

                let sf = SourceFile {
                    keccak256: None,
                    urls: vec![path.to_string_lossy().to_string()],
                };
                sources.insert(filename, sf);
            }
        }

        let mut output_selection = HashMap::new();

        let mut contract_output = HashMap::new();
        contract_output.insert(
            "*".to_string(),
            vec![
                OutputSelection::Abi,
                OutputSelection::EvmBytecode,
                OutputSelection::EvmGasEstimates,
                OutputSelection::EvmMethodIdentifiers,
                OutputSelection::EvmBytecodeSourceMap,
                OutputSelection::EvmDeployedBytecode,
            ],
        );

        output_selection.insert("*".to_string(), contract_output);

        let profile_s = env::var("PROFILE")?;
        let pt = ProfileType::from_str(&profile_s).ok_or(Error::UnknownProfileType)?;
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
                stop_after: input::StopAfter::Parsing,
                remappings: vec![],
                optimizer,
                evm_version: self.snapper.solidity.evm_version,
                via_ir: self.snapper.solidity.via_ir,
                debug: SettingsDebug {
                    revert_strings,
                    debug_info: vec![DebugInfo::All],
                },
                metadata: None,
                libraries: self.snapper.library,
                output_selection,
                model_checker: None,
            },
        };

        let in_data = serde_json::to_string(&input)?;

        let mut command = Command::new(self.solc_path)
            .arg("--standard-json")
            .spawn()?;

        let mut stdio = command.stdin.take().ok_or(Error::FailedToGetStdio)?;

        stdio.write(in_data.as_bytes()).await?;

        let output = command.wait_with_output().await?;

        let res: CompilerOutput = serde_json::from_slice(&output.stdout)?;

        println!("{:#?}", res);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use crate::{Config, Solc};

    #[test]
    fn test() {
        let config = Config {
            upstream: None,
            out_dir: Some("../target".to_string()),
            snapper_file: Some("../cargo-snapper/assets/Snapper.toml".to_string()),
        };

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let _solc = Solc::new_with_config(config).await.unwrap();
        });
    }
}
