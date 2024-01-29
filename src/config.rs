use crate::handle_parse_all_files::SolidityABI;
use crate::typescript_boilerplate::{ethers_boilerplate, viem_boilerplate};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;
use std::result::Result;
use walkdir::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Framework {
    Foundry,
    Hardhat,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modes {
    EthersV5,
    Viem,
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

//serializer
fn serialize_framework<S>(framework: &Framework, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match framework {
        Framework::Foundry => serializer.serialize_str("foundry"),
        Framework::Hardhat => serializer.serialize_str("hardhat"),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JasperConfig {
    #[serde(rename = "outputFolder")]
    pub output_folder: String,
    #[serde(rename = "typescript")]
    pub typescript: bool,
    #[serde(
        rename = "framework",
        deserialize_with = "deserialize_framework",
        serialize_with = "serialize_framework"
    )]
    pub framework: Framework,
    #[serde(rename = "contractsPath")]
    pub contracts_path: String,
    #[serde(rename = "excludedFiles")]
    pub excluded_files: Vec<String>,
    #[serde(rename = "extraIncludedFiles")]
    pub extra_included_files: Vec<String>,
    pub modes: Vec<Modes>,
}

pub struct JasperConfigOutput {
    pub output_folder: String,
    pub typescript: bool,
    pub all_abi_files: Vec<String>,
    pub all_solidity_files: Vec<String>,
    pub framework: Framework,
    pub modes: Vec<Modes>,
}

pub fn read_config() -> JasperConfigOutput {
    generate_basic_config();
    let config_file =
        fs::read_to_string("jasper-config.json").expect("Could not read jasper-config.json");
    let config: JasperConfig =
        serde_json::from_str(&config_file).expect("Could not parse jasper-config.json");
    let all_abi_files = get_all_abi_files(&config);

    let all_solidity_files = get_all_solidity_files(&config);

    return JasperConfigOutput {
        output_folder: config.output_folder,
        typescript: config.typescript,
        all_abi_files,
        all_solidity_files,
        framework: config.framework,
        modes: config.modes,
    };
}

// pub struct FoundryBuildOutput {
//     abi: Vec<SolidityABI>,

// }
fn get_all_abi_files(config: &JasperConfig) -> Vec<String> {
    let out_folder = match config.framework {
        Framework::Foundry => "./out/",
        Framework::Hardhat => "./artifacts/",
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
        .filter(|e| {
            e.path().to_str().unwrap().ends_with(".sol")
                && !e.path().to_str().unwrap().ends_with(".t.sol")
        })
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    all_solidity_files.extend(config.extra_included_files.clone());

    return all_solidity_files;
}

#[derive(Serialize, Deserialize)]
struct FileWithABI {
    abi: Vec<SolidityABI>,
}
pub fn parse_abi_file(
    abi_file_path: &str,
    config: &JasperConfigOutput,
) -> Option<Vec<SolidityABI>> {
    let abi_json = std::fs::read_to_string(abi_file_path);
    match abi_json {
        Err(_) => {
            panic!("Could not read abi file {}", abi_file_path);
        }
        _ => {}
    }
    let abi_json = abi_json.unwrap();

    match config.framework {
        Framework::Foundry => {
            // println!("{}",abi_json);
            let abi: Result<FileWithABI, serde_json::Error> = serde_json::from_str(&abi_json);
            match abi {
                Ok(abi) => {
                    return Some(abi.abi);
                }
                Err(_) => {
                    // println!("Could not parse abi file {}", abi_file_path);
                    return None;
                }
            }
        }
        Framework::Hardhat => {
            let abi: Result<FileWithABI, serde_json::Error> = serde_json::from_str(&abi_json);
            match abi {
                Ok(abi) => {
                    return Some(abi.abi);
                }
                Err(_) => {
                    // println!("Could not parse abi file {}", abi_file_path);
                    return None;
                }
            }
        }
    }
}

pub fn generate_basic_config() {
    let path = "jasper-config.json";
    //If it doesent exist, create it
    let exists = std::path::Path::new(path).exists();
    if exists {
        return;
    }
    if !exists {
        let config = JasperConfig {
            output_folder: "./jasper-bindings".to_string(),
            typescript: true,
            framework: Framework::Foundry,
            contracts_path: "./src".to_string(),
            excluded_files: vec!["./contracts/Migrations.sol".to_string()],
            extra_included_files: vec![],
            modes: vec![Modes::EthersV5],
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write("./jasper-config.json", config_json).unwrap();
        println!("Generated config in jasper-config.json");
    }
}

pub fn generate_mode_files(config: &JasperConfigOutput) {
    let modes = config.modes.clone();
    for mode in modes {
        match mode {
            Modes::EthersV5 => {
                let filename = "jasperParseErrorEthers.ts";
                let file_contents = ethers_boilerplate();
                let path = format!("{}/{}", config.output_folder, filename);
                let path = path.replace("//", "/");
                fs::write(path, file_contents).expect("Unable to write file");
            }

            Modes::Viem => {
                let filename = "jasperParseErrorViem.ts";
                let file_contents = viem_boilerplate();
                let path = format!("{}/{}", config.output_folder, filename);
                let path = path.replace("//", "/");
                fs::write(path, file_contents).expect("Unable to write file");
            }
        }
    }
}

pub fn create_output_folder_if_not_exists(config: &JasperConfigOutput) {
    let output_folder = config.output_folder.clone();
    let exists = std::path::Path::new(&output_folder).exists();
    if !exists {
        std::fs::create_dir_all(&output_folder).expect("Could not create output folder");
    }
}
