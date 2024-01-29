use crate::constants;
use crate::find_selector;
use crate::parse_sol_file::{parse_sol_file, SolidityMessageAndArgs};
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::read_config::{parse_abi_file,JasperConfigOutput};
#[derive(Serialize, Deserialize, Clone)]
pub struct ABIInput {
    name: Option<String>,
    #[serde(rename = "type")]
    _type: Option<String>,
    #[serde(rename = "internalType")]
    internal_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SolidityABI {
    #[serde(rename = "type")]
    _type: String,
    name: Option<String>,
    inputs: Option<Vec<ABIInput>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Output {
    #[serde(rename = "errorName")]
    error_name: String,
    #[serde(rename = "solidityMessageAndArgs")]
    solidity_message_and_args: SolidityMessageAndArgs,
}
pub fn prepare_error_from_abi_input(input: &SolidityABI) -> (String,Vec<ABIInput>) {
    let mut error = String::new();
    //Name(input[0].type, input[1].type, ...)
    error.push_str(&input.name.clone().unwrap());
    error.push('(');
    let mut input_types = String::new();
    for input in input.inputs.clone().unwrap() {
        input_types.push_str(&input._type.unwrap());
        input_types.push(',');
    }
    //remove the last comma
    input_types.pop();
    error.push_str(&input_types);
    error.push(')');
    return (error,input.inputs.unwrap());
}
pub fn handle_parse_all_files(abi_files: Vec<String>, solidity_files: Vec<String>,config:&JasperConfigOutput) {
    let (selectors, prepared_errors) = get_prepared_errors_from_abi_files(abi_files,config);

    // For each solidity file,aggregate the solidity file
    let error_to_message_map: HashMap<String, SolidityMessageAndArgs> = solidity_files
        .par_iter()
        .map(|sol_file_path| {
            let _error_to_message_map: HashMap<String, SolidityMessageAndArgs> =
                parse_sol_file(sol_file_path.as_str())
                    .unwrap_or_else(|_| panic!("Could not parse the solidity file"));
            return _error_to_message_map;
        })
        .reduce(HashMap::new, |mut acc, elem| {
            acc.extend(elem);
            return acc;
        });

    //loop over all error to error_to_error_message map and print
    //print the length of the map
    for (key, value) in error_to_message_map.iter() {
        println!("{}:{}", key, value.error_message);
    }

    //Loop over all the selectors and prepared errors
    let output = selectors
        .into_iter()
        .zip(prepared_errors.into_iter())
        .map(|(selector, error)| {
            let key_in_error_to_message_map = error.split("(").collect::<Vec<&str>>()[0];
            let error_message = match error_to_message_map.get(key_in_error_to_message_map) {
                Some(message) => {
                    // Deserialize the JSON string to a regular Rust string
                    let cleaned_error_message =
                        serde_json::from_str::<String>(message.error_message.as_str())
                            .unwrap_or_else(|_| "".to_string());
                    SolidityMessageAndArgs {
                        error_message: cleaned_error_message,
                        args: message.args.clone(),
                    }
                }
                None => SolidityMessageAndArgs {
                    error_message: "".to_owned(),
                    args: vec![],
                },
            };

            let output = Output {
                error_name: error,
                solidity_message_and_args: error_message,
            };

            (selector, output)
        })
        .collect::<HashMap<String, Output>>();

    //Save the map
    let map_json = serde_json::to_string_pretty(&output).unwrap();

    let typescript_prefix = "export const errors:JasperObject = ";
    let typescript_suffix = ";";
    let typescript =
        constants::PREFIX_TEXT.to_owned() + typescript_prefix + &map_json + typescript_suffix;
    std::fs::write("ts/errors.ts", typescript).unwrap();

    //We need to make an aggregate map
    // let mut aggregate_map:HashMap<String,Output> = HashMap::new();

    // let typescript_prefix = "export const errors = ";
    // let typescript_suffix = ";";
    // let typescript =
    //     constants::PREFIX_TEXT.to_owned() + typescript_prefix + &map_json + typescript_suffix;
    // //Make it pretty
    // std::fs::write("ts/errors2.ts", typescript).unwrap();
}

pub fn get_prepared_errors_from_abi_files(abi_files: Vec<String>,config:&JasperConfigOutput) -> (Vec<String>, Vec<String>) {
    let mut prepared_errors: Vec<String> = Vec::new();
    let mut selectors: Vec<String> = Vec::new();

    for (i, abi_fn) in abi_files.iter().enumerate() {
        //read in the abi file
        let abi = parse_abi_file(abi_fn,config);

        //Find only the errors
        let errors: Vec<SolidityABI> = abi
            .into_iter()
            .filter(|x| x._type == "error")
            .collect::<Vec<SolidityABI>>();

        //A list of errors as strings
        let mut prepared_errors_from_file = errors
            .clone().into_iter()
            .map(|x| prepare_error_from_abi_input(&x).0)
            .collect::<Vec<String>>();
        let abi_inputs = errors
            .into_iter()
            .map(|x| prepare_error_from_abi_input(&x).1)
            .collect::<Vec<Vec<ABIInput>>>();

        //A vector of all the selectors
        let mut selectors_from_file = prepared_errors_from_file
            .clone()
            .into_iter()
            .map(|x| find_selector::find_selector(&x))
            .collect::<Vec<String>>();

        //Add the selectors and errors to the main vectors
        selectors.append(&mut selectors_from_file);
        prepared_errors.append(&mut prepared_errors_from_file);
    }

    // });

    //Return the selectors and the prepared errors
    return (selectors, prepared_errors);
}
// pub fn handle_single_files(abi_file_path:String,sol_file_path:String) -> HashMap<String,Output> {

//     //
//     let abi_json = std::fs::read_to_string(abi_file_path);
//     let abi_json = abi_json.unwrap();
//     let abi: Vec<SolidityABI> = serde_json::from_str(&abi_json).unwrap();

//     //Find only the errors
//     let errors: Vec<SolidityABI> = abi
//         .into_iter()
//         .filter(|x| x._type == "error")
//         .collect::<Vec<SolidityABI>>();

//     //Find all the errors
//     let cleaned_errors: Vec<String> = errors
//         .into_iter()
//         .map(|x| prepare_error_from_abi_input(&x))
//         .collect::<Vec<String>>();

//     //Make a hashmap of selector -> error
//     let selectors = cleaned_errors
//         .clone()
//         .into_iter()
//         .map(|x| find_selector::find_selector(&x))
//         .collect::<Vec<String>>();

//     let error_to_message_map: HashMap<String, SolidityMessageAndArgs> =
//         parse_sol_file(sol_file_path.as_str()).unwrap_or_else(|_| panic!("Could not parse the solidity file"));

//     let output: HashMap<String, Output> = selectors
//         .into_iter()
//         .zip(cleaned_errors.into_iter())
//         .map(|(selector, error)| {
//             let key_in_error_to_message_map = error.split("(").collect::<Vec<&str>>()[0];
//             let error_message = match error_to_message_map.get(key_in_error_to_message_map) {
//                 Some(message) => {
//                     // Deserialize the JSON string to a regular Rust string
//                     let cleaned_error_message =
//                         serde_json::from_str::<String>(message.error_message.as_str())
//                             .unwrap_or_else(|_| "".to_string());
//                     SolidityMessageAndArgs {
//                         error_message: cleaned_error_message,
//                         args: message.args.clone(),
//                     }
//                 }
//                 None => SolidityMessageAndArgs {
//                     error_message: "".to_owned(),
//                     args: vec![],
//                 },
//             };

//             let output = Output {
//                 error_name: error,
//                 solidity_message_and_args: error_message,
//             };

//             (selector, output)
//         })
//         .collect::<HashMap<String, Output>>();

//     //return the map
//     return output;

// }
