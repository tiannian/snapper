use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{anyhow, Result};
use snapper_core::{ProfileType, SnapperFile};

use crate::{
    input::{
        self, DebugInfo, Optimizer, OptimizerDetails, OutputSelection, RevertStrings,
        SettingsDebug, SourceFile, YulDetails,
    },
    utils,
    version::Platform,
    CompilerInput, CompilerOutput, CompilerVersions,
};

pub struct Solc {
    pub snapper: SnapperFile,
    solc_path: PathBuf,
}

impl Solc {
    /// New a solc instance
    ///
    /// Notice: this function only can be call in `build.rs`
    pub fn new<P: AsRef<Path>>(out_dir: P, upstream: Option<&str>, snapper: &str) -> Result<Self> {
        let versions = if let Some(upstream) = upstream {
            CompilerVersions::load_from(upstream)?
        } else {
            CompilerVersions::load()?
        };

        let snapper = utils::load_snapper_file(snapper)?;

        let platform = Platform::from_target().ok_or(anyhow!("No support platform"))?;

        fs::create_dir_all(&out_dir)?;

        let solc_path = utils::solc_path(out_dir.as_ref(), &snapper.solidity.version)?;

        if !solc_path.exists() {
            versions.download(&snapper.solidity.version, &platform, &solc_path)?;
        }

        Ok(Self { snapper, solc_path })
    }

    pub fn compile<P: AsRef<Path>>(
        &self,
        file: P,
        profile_type: &ProfileType,
        out_dir: P,
    ) -> Result<Vec<String>> {
        let mut sources = HashMap::new();

        let file = file.as_ref();

        let filename = file
            .file_name()
            .ok_or(anyhow!("Failed to parse filename"))?
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
            .ok_or(anyhow!("Failed to get stdin"))?
            .write_all(in_data.as_bytes())?;

        let output = command.wait_with_output()?;

        let res: CompilerOutput = serde_json::from_slice(&output.stdout)?;

        if !res.errors.is_empty() {
            println!("{:?}", res.errors);
            panic!("Solidity compile error");
        }

        let res = if let Some(contracts) = res
            .contracts
            .ok_or(anyhow!("No target contract output"))?
            .get(&filename)
        {
            let mut res = Vec::with_capacity(contracts.len());

            for (name, contract) in contracts.iter() {
                res.push(name.clone());

                let out_dir = out_dir.as_ref();

                let contract_dir = out_dir.join(&filename);
                fs::create_dir_all(&contract_dir)?;

                let abi = &contract.abi;
                let bytecode = &contract.evm.bytecode.object;
                let opcodes = &contract.evm.bytecode.opcodes.trim();
                let sourcemap = &contract.evm.bytecode.source_map.trim();
                let gas = &contract.evm.gas_estimates;

                let mut file = File::create(contract_dir.join(format!("{name}.abi.json")))?;
                file.write_all(serde_json::to_string(abi)?.as_bytes())?;

                let mut file = File::create(contract_dir.join(format!("{name}.bytecode")))?;
                file.write_all(bytecode)?;

                let mut file = File::create(contract_dir.join(format!("{name}.opcodes")))?;
                file.write_all(opcodes.as_bytes())?;

                let mut file = File::create(contract_dir.join(format!("{name}.gas.json")))?;
                file.write_all(serde_json::to_string(gas)?.as_bytes())?;

                let mut file = File::create(contract_dir.join(format!("{name}.sourcemap")))?;
                file.write_all(sourcemap.as_bytes())?;
            }

            res
        } else {
            vec![]
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use snapper_core::ProfileType;

    use crate::Solc;

    #[test]
    fn test() {
        let out_dir = "../target";
        let snapper_file = "../cargo-snapper/assets/Snapper.toml";

        let sf = std::fs::read_to_string(snapper_file).unwrap();

        let solc = Solc::new(out_dir, None, &sf).unwrap();
        solc.compile(
            "contracts/Lock.sol",
            &ProfileType::Debug,
            "../target/solc-test/",
        )
        .unwrap();
    }
}
