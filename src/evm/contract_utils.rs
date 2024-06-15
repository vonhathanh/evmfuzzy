use bytes::Bytes;
/// Load contract from file system or remote
use glob::glob;
use revm_primitives::Env;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use super::{
    types::{EVMAddress, EVMFuzzState},
    vm::EVMState,
};
use itertools::Itertools;
use tracing::{debug, error};

// to use this address, call rand_utils::fixed_address(FIX_DEPLOYER)
pub static FIX_DEPLOYER: &str = "8b21e662154b4bbc1ec0754d0238875fe3d22fa6";
pub static FOUNDRY_DEPLOYER: &str = "1804c8AB1F12E6bbf3894d4083f33e07309d1f38";
pub static FOUNDRY_SETUP_ADDR: &str = "e1A425f1AC34A8a441566f93c82dD730639c8510";

#[derive(Debug, Default)]
pub struct SetupData {
    pub evmstate: EVMState,
    pub env: Env,
    pub code: HashMap<EVMAddress, Bytes>,
    // Foundry specific
    // pub excluded_contracts: Vec<EVMAddress>,
    // pub excluded_senders: Vec<EVMAddress>,
    // pub target_contracts: Vec<EVMAddress>,
    // pub target_senders: Vec<EVMAddress>,
    // pub target_selectors: HashMap<EVMAddress, Vec<Vec<u8>>>,

    // Flashloan specific
    // pub v2_pairs: Vec<EVMAddress>,
    // pub constant_pairs: Vec<ConstantPairMetadata>,

    // pub onchain_middleware: Option<OnChain>,
}

impl Clone for SetupData {
    fn clone(&self) -> Self {
        Self {
            evmstate: self.evmstate.clone(),
            env: self.env.clone(),
            code: self.code.clone(),
            // excluded_contracts: self.excluded_contracts.clone(),
            // excluded_senders: self.excluded_senders.clone(),
            // target_contracts: self.target_contracts.clone(),
            // target_senders: self.target_senders.clone(),
            // target_selectors: self.target_selectors.clone(),
            // v2_pairs: self.v2_pairs.clone(),
            // constant_pairs: self.constant_pairs.clone(),
            // onchain_middleware: self.onchain_middleware.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABIConfig {
    pub abi: String,
    pub function: [u8; 4],
    pub function_name: String,
    pub is_static: bool,
    pub is_payable: bool,
    pub is_constructor: bool,
    #[serde(default)]
    pub should_add_corpus: bool,
}

#[derive(Debug, Clone)]
pub struct ABIInfo {
    pub source: String,
    pub abi: Vec<ABIConfig>,
}

#[derive(Debug, Clone)]
pub struct ContractInfo {
    pub name: String,
    pub code: Vec<u8>,
    pub abi: Vec<ABIConfig>,
    pub is_code_deployed: bool,
    pub constructor_args: Vec<u8>,
    pub deployed_address: EVMAddress,
    // pub build_artifact: Option<BuildJobResult>,
    pub files: Vec<(String, String)>, // (filename, content)
    pub source_map_replacements: Option<Vec<(String, String)>>,
    pub raw_source_map: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContractLoader {
    pub contracts: Vec<ContractInfo>,
    pub abis: Vec<ABIInfo>,
    pub setup_data: Option<SetupData>,
}

impl ContractLoader {
    // pub fn from_prefix(
    //     prefix: &str,
    //     state: &mut EVMFuzzState,
    //     proxy_deploy_codes: &Vec<String>,
    //     constructor_args: &[String],
    //     files: Vec<(String, String)>,
    //     source_map_replacements: Option<Vec<(String, String)>>,
    //     raw_source_maps: HashMap<String, String>, // contract name -> raw source map
    // ) -> Self {
    //     let contract_name = prefix.split('/').last().unwrap().replace('*', "");

    //     // get constructor args
    //     let constructor_args_in_bytes: Vec<u8> = Self::constructor_args_encode(constructor_args);

    //     // create dummy contract info
    //     let mut contract_result = ContractInfo {
    //         name: prefix.to_string(),
    //         code: vec![],
    //         abi: vec![],
    //         is_code_deployed: false,
    //         constructor_args: constructor_args_in_bytes,
    //         deployed_address: generate_random_address(state),
    //         build_artifact: None,
    //         files,
    //         source_map_replacements,
    //         raw_source_map: raw_source_maps.get(&contract_name).cloned(),
    //     };
    //     let mut abi_result = ABIInfo {
    //         source: prefix.to_string(),
    //         abi: vec![],
    //     };

    //     debug!("Loading contract {}", prefix);

    //     // Load contract, ABI, and address from file
    //     for i in glob(prefix).expect("not such path for prefix") {
    //         match i {
    //             Ok(path) => {
    //                 if path.to_str().unwrap().ends_with(".abi") {
    //                     // this is an ABI file
    //                     abi_result.abi = Self::parse_abi(&path);
    //                     contract_result.abi = abi_result.abi.clone();
    //                     // debug!("ABI: {:?}", result.abis);
    //                 } else if path.to_str().unwrap().ends_with(".bin") {
    //                     // this is an BIN file
    //                     contract_result.code = Self::parse_hex_file(&path);
    //                 } else if path.to_str().unwrap().ends_with(".address") {
    //                     // this is deployed address
    //                     contract_result
    //                         .deployed_address
    //                         .0
    //                         .clone_from_slice(Self::parse_hex_file(&path).as_slice());
    //                 } else {
    //                     debug!("Found unknown file: {:?}", path.display())
    //                 }
    //             }
    //             Err(e) => error!("{:?}", e),
    //         }
    //     }

    //     if let Some(abi) = abi_result.abi.iter().find(|abi| abi.is_constructor) {
    //         let mut abi_instance =
    //             get_abi_type_boxed_with_address(&abi.abi, fixed_address(FIX_DEPLOYER).0.to_vec());
    //         abi_instance.set_func_with_signature(abi.function, &abi.function_name, &abi.abi);
    //         if contract_result.constructor_args.is_empty() {
    //             debug!("No constructor args found, using default constructor args");
    //             contract_result.constructor_args = abi_instance.get().get_bytes();
    //         }
    //         // debug!("Constructor args: {:?}", result.constructor_args);
    //         contract_result
    //             .code
    //             .extend(contract_result.constructor_args.clone());
    //     } else {
    //         debug!("No constructor in ABI found, skipping");
    //     }

    //     // now check if contract is deployed through proxy by checking function
    //     // signatures if it is, then we use the new bytecode from proxy
    //     // todo: find a better way to do this
    //     let current_code = hex::encode(&contract_result.code);
    //     for deployed_code in proxy_deploy_codes {
    //         // if deploy_code startwiths '0x' then remove it
    //         let deployed_code = if let Some(stripped) = deployed_code.strip_prefix("0x") {
    //             stripped
    //         } else {
    //             deployed_code
    //         };

    //         // match all function signatures, compare sigs between our code and deployed
    //         // code from proxy
    //         let deployed_code_sig: Vec<[u8; 4]> = extract_sig_from_contract(deployed_code);
    //         let current_code_sig: Vec<[u8; 4]> = extract_sig_from_contract(&current_code);

    //         // compare deployed_code_sig and current_code_sig
    //         if deployed_code_sig.len() == current_code_sig.len() {
    //             let mut is_match = true;
    //             for i in 0..deployed_code_sig.len() {
    //                 if deployed_code_sig[i] != current_code_sig[i] {
    //                     is_match = false;
    //                     break;
    //                 }
    //             }
    //             if is_match {
    //                 contract_result.code =
    //                     hex::decode(deployed_code).expect("Failed to parse deploy code");
    //             }
    //         }
    //     }
    //     Self {
    //         contracts: if !contract_result.code.is_empty() {
    //             vec![contract_result]
    //         } else {
    //             vec![]
    //         },
    //         abis: vec![abi_result],
    //         setup_data: None,
    //     }
    // }

    pub fn from_glob(
        p: &str,
        // state: &mut EVMFuzzState,
        // proxy_deploy_codes: &Vec<String>,
        // constructor_args_map: &HashMap<String, Vec<String>>,
        // base_path: String,
        // additional_path: Option<String>,
    ) -> Self {
        // let mut prefix_file_count: HashMap<String, u8> = HashMap::new();
        // let mut contract_combined_json_info = None;
        // for i in glob(p).expect("not such folder") {
        //     match i {
        //         Ok(path) => {
        //             let path_str = path.to_str().unwrap();
        //             if path_str.ends_with(".abi") {
        //                 // skip FuzzLand.abi
        //                 if path_str.ends_with("FuzzLand.abi") {
        //                     continue;
        //                 }
        //                 *prefix_file_count
        //                     .entry(path_str.replace(".abi", "").clone())
        //                     .or_insert(0) += 1;
        //             } else if path_str.ends_with(".bin") {
        //                 // skip FuzzLand.bin
        //                 if path_str.ends_with("FuzzLand.bin") {
        //                     continue;
        //                 }
        //                 *prefix_file_count
        //                     .entry(path_str.replace(".bin", "").clone())
        //                     .or_insert(0) += 1;
        //             } else if path_str.ends_with("combined.json") {
        //                 contract_combined_json_info = Some(path_str.to_string());
        //             } else {
        //                 debug!("Found unknown file in folder: {:?}", path.display())
        //             }
        //         }
        //         Err(e) => error!("{:?}", e),
        //     }
        // }

        // let (files, source_map_replacements, raw_sourcemaps) = match contract_combined_json_info {
        //     Some(json_filename) => {
        //         let mut json_file = File::open(json_filename).unwrap();
        //         let mut buf = String::new();
        //         json_file.read_to_string(&mut buf).unwrap();

        //         extract_combined_json(buf, base_path, additional_path)
        //     }
        //     None => (vec![], None, HashMap::new()),
        // };

        // let mut contracts: Vec<ContractInfo> = vec![];
        // let mut abis: Vec<ABIInfo> = vec![];
        // for (prefix, count) in prefix_file_count
        //     .iter()
        //     .sorted_by_key(|(k, _)| <&String>::clone(k))
        // {
        //     let p = prefix.to_string();
        //     if *count > 0 {
        //         let mut constructor_args: Vec<String> = vec![];
        //         for (k, v) in constructor_args_map.iter() {
        //             let components: Vec<&str> = p.split('/').collect();
        //             if let Some(last_component) = components.last() {
        //                 if last_component == k {
        //                     constructor_args = v.clone();
        //                 }
        //             }
        //         }
        //         let prefix_loader = Self::from_prefix(
        //             (prefix.to_owned() + &String::from('*')).as_str(),
        //             state,
        //             proxy_deploy_codes,
        //             &constructor_args,
        //             files.clone(),
        //             source_map_replacements.clone(),
        //             raw_sourcemaps.clone(),
        //         );
        //         prefix_loader
        //             .contracts
        //             .iter()
        //             .for_each(|c| contracts.push(c.clone()));
        //         prefix_loader.abis.iter().for_each(|a| abis.push(a.clone()));
        //     }
        // }

        ContractLoader {
            contracts: vec![],
            abis: vec![],
            setup_data: None,
        }
    }
}

type CombinedJsonOutput = (
    Vec<(String, String)>,
    Option<Vec<(String, String)>>,
    HashMap<String, String>,
);
// Return (vec_files, vec_replacements, vec_raw_source_map)
pub fn extract_combined_json(
    json: String,
    target_path: String,
    base_path: Option<String>,
) -> CombinedJsonOutput {
    let map_json = serde_json::from_str::<serde_json::Value>(&json).unwrap();
    let contracts = map_json["contracts"]
        .as_object()
        .expect("contracts not found");
    let file_list = map_json["sourceList"]
        .as_array()
        .expect("sourceList not found")
        .iter()
        .map(|x| x.as_str().expect("sourceList is not string").to_string())
        .collect::<Vec<String>>();

    // the base_path can be either absolute or relative
    // if absolute, then we use it as is
    // if relative, then we concat it with target_path
    let root_path = match base_path.clone() {
        Some(base_path) => {
            let base_pathbuf = PathBuf::from(base_path.clone());
            if !base_pathbuf.is_absolute() {
                // Remove the last "*" char and join with target_path
                PathBuf::from(
                    format!(
                        "{}{}",
                        target_path.clone().strip_suffix('*').unwrap(),
                        base_path
                    )
                    .to_string(),
                )
            } else {
                PathBuf::from(base_path)
            }
        }
        // Remove the last "*" char
        None => PathBuf::from(target_path.clone().strip_suffix('*').unwrap()),
    };

    let mut vec_filenames = Vec::<(String, String)>::new();
    for filename in &file_list {
        // construct the full path
        let mut pathbuf = root_path.clone();
        pathbuf.push(filename);
        // read the file
        let mut file = match File::open(&pathbuf) {
            Ok(file) => file,
            Err(_) => {
                let mut pathbuf = PathBuf::from(base_path.clone().unwrap_or(".".to_string()));
                pathbuf.push(filename);
                File::open(&pathbuf)
                    .unwrap_or_else(|_| panic!("cannot open file {}", pathbuf.display()))
            }
        };
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        // push to vec_filenames
        vec_filenames.push((filename.clone(), buf));
    }

    let mut raw_source_maps = HashMap::<String, String>::new();
    for (contract_name, contract_info) in contracts {
        let splitter = contract_name.split(':').collect::<Vec<&str>>();
        let contract_name = splitter.last().unwrap().to_string();

        let srcmap_runtime = contract_info["srcmap-runtime"]
            .as_str()
            .expect("srcmap-runtime not found")
            .to_string();

        raw_source_maps.insert(contract_name.clone(), srcmap_runtime);
    }

    (vec_filenames, None, raw_source_maps)
}
