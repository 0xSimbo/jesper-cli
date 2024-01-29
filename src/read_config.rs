use crate::handle_parse_all_files::SolidityABI;
use serde::{Deserialize, Serialize,Deserializer};
use std::fs;
use walkdir::*;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum Framework {
    Foundry,
    Hardhat,
}
// Custom deserializer for the Framework enum
fn deserialize_framework<'de, D>(deserializer: D) -> Result<Framework, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    match s.as_str() {
        "foundry" => Ok(Framework::Foundry),
        "hardhat" => Ok(Framework::Hardhat),
        _ => Err(serde::de::Error::custom("invalid framework")),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JasperConfig {
    #[serde(rename = "outputFolder")]
    pub output_folder: String,
    #[serde(rename = "typescript")]
    pub typescript: bool,
    #[serde(rename = "framework", deserialize_with = "deserialize_framework")]
    pub framework: Framework,
    #[serde(rename = "contractsPath")]
    pub contracts_path: String,
    #[serde(rename = "excludedFiles")]
    pub excluded_files: Vec<String>,
    #[serde(rename = "extraIncludedFiles")]
    pub extra_included_files: Vec<String>,
}

pub struct JasperConfigOutput {
    pub output_folder: String,
    pub typescript: bool,
    pub all_abi_files: Vec<String>,
    pub all_solidity_files: Vec<String>,
    pub framework: Framework,
}

pub fn read_config() -> JasperConfigOutput {
    let config_file = fs::read_to_string("jasper-config.json").expect("Could not read jasper.json");
    let config: JasperConfig =
        serde_json::from_str(&config_file).expect("Could not parse jasper.json");
    let all_abi_files = get_all_abi_files(&config);

    let all_solidity_files = get_all_solidity_files(&config);

    return JasperConfigOutput {
        output_folder: config.output_folder,
        typescript: config.typescript,
        all_abi_files,
        all_solidity_files,
        framework: config.framework,
    };
}

// pub struct FoundryBuildOutput {
//     abi: Vec<SolidityABI>,

// }
fn get_all_abi_files(config: &JasperConfig) -> Vec<String> {
    let out_folder = match config.framework {
        Framework::Foundry =>  "./out/",
        Framework::Hardhat =>  "./artifacts/",
    };

    //Recursively find all the abi files in the contracts folder
    let all_abi_files = walkdir::WalkDir::new(out_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.path().to_str().unwrap().ends_with(".json"))
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    return all_abi_files;
}

fn get_all_solidity_files(config: &JasperConfig) -> Vec<String> {
    let mut all_solidity_files = walkdir::WalkDir::new(config.contracts_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.path().to_str().unwrap().ends_with(".sol") && ! e.path().to_str().unwrap().ends_with(".t.sol") )
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    all_solidity_files.extend(config.extra_included_files.clone());

    return all_solidity_files;
}

#[derive(Serialize, Deserialize)]
struct FileWithABI {
    abi: Vec<SolidityABI>,
}
pub fn parse_abi_file(abi_file_path:&str,config:&JasperConfigOutput) -> Vec<SolidityABI> {
    match config.framework {
        Framework::Foundry => {
            let abi_json = std::fs::read_to_string(abi_file_path);
     

            let abi_json = abi_json.unwrap();
            // println!("{}",abi_json);
            let abi: FileWithABI = serde_json::from_str(&abi_json).unwrap();
            return abi.abi;
        },
        Framework::Hardhat => {
            let abi_json = std::fs::read_to_string(abi_file_path);
            let abi_json = abi_json.unwrap();
            let abi: Vec<SolidityABI> = serde_json::from_str(&abi_json).unwrap();
            return abi;
        }
    }
}