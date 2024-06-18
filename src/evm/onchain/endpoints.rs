use reqwest::{blocking, header::HeaderMap};
use retry::{delay::Fixed, retry_with_index, OperationResult};
use revm_primitives::Bytecode;
use std::{collections::HashMap, env, hash::{DefaultHasher, Hash, Hasher}, str::FromStr, sync::Arc, time::Duration};

use crate::{
    cache::{Cache, FileSystemCache},
    evm::{
        tokens::TokenContext,
        types::{EVMAddress, EVMU256},
    },
};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use tracing::{debug, error, info, warn};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum Chain {
    ETH,
    GOERLI,
    SEPOLIA,
    BSC,
    CHAPEL,
    POLYGON,
    MUMBAI,
    FANTOM,
    AVALANCHE,
    OPTIMISM,
    ARBITRUM,
    GNOSIS,
    BASE,
    CELO,
    ZKEVM,
    ZkevmTestnet,
    BLAST,
    LOCAL,
}

impl FromStr for Chain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eth" | "mainnet" => Ok(Self::ETH),
            "goerli" => Ok(Self::GOERLI),
            "sepolia" => Ok(Self::SEPOLIA),
            "bsc" => Ok(Self::BSC),
            "chapel" => Ok(Self::CHAPEL),
            "polygon" => Ok(Self::POLYGON),
            "mumbai" => Ok(Self::MUMBAI),
            "fantom" => Ok(Self::FANTOM),
            "avalanche" => Ok(Self::AVALANCHE),
            "optimism" => Ok(Self::OPTIMISM),
            "arbitrum" => Ok(Self::ARBITRUM),
            "gnosis" => Ok(Self::GNOSIS),
            "base" => Ok(Self::BASE),
            "celo" => Ok(Self::CELO),
            "zkevm" => Ok(Self::ZKEVM),
            "zkevm_testnet" => Ok(Self::ZkevmTestnet),
            "blast" => Ok(Self::BLAST),
            "local" => Ok(Self::LOCAL),
            _ => Err(()),
        }
    }
}

impl Chain {
    pub fn new_with_rpc_url(rpc_url: &str) -> Result<Self> {
        let client = blocking::Client::new();
        let body = json!({"method":"eth_chainId","params":[],"id":1,"jsonrpc":"2.0"});
        let resp: Value = client
            .post(rpc_url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()?
            .json()?;

        let chain_id = resp
            .get("result")
            .and_then(|result| result.as_str())
            .and_then(|result| u64::from_str_radix(result.trim_start_matches("0x"), 16).ok())
            .ok_or_else(|| anyhow!("Unknown chain id: {}", rpc_url))?;

        // Use rpc_url instead of the default one
        env::set_var("ETH_RPC_URL", rpc_url);

        Ok(match chain_id {
            1 => Self::ETH,
            5 => Self::GOERLI,
            11155111 => Self::SEPOLIA,
            56 => Self::BSC,
            97 => Self::CHAPEL,
            137 => Self::POLYGON,
            80001 => Self::MUMBAI,
            250 => Self::FANTOM,
            43114 => Self::AVALANCHE,
            10 => Self::OPTIMISM,
            42161 => Self::ARBITRUM,
            100 => Self::GNOSIS,
            8453 => Self::BASE,
            42220 => Self::CELO,
            1101 => Self::ZKEVM,
            1442 => Self::ZkevmTestnet,
            81457 => Self::BLAST,
            31337 => Self::LOCAL,
            _ => return Err(anyhow!("Unknown chain id: {}", chain_id)),
        })
    }

    pub fn get_chain_id(&self) -> u32 {
        match self {
            Chain::ETH => 1,
            Chain::GOERLI => 5,
            Chain::SEPOLIA => 11155111,
            Chain::BSC => 56,
            Chain::CHAPEL => 97,
            Chain::POLYGON => 137,
            Chain::MUMBAI => 80001,
            Chain::FANTOM => 250,
            Chain::AVALANCHE => 43114,
            Chain::OPTIMISM => 10,
            Chain::ARBITRUM => 42161,
            Chain::GNOSIS => 100,
            Chain::BASE => 8453,
            Chain::CELO => 42220,
            Chain::ZKEVM => 1101,
            Chain::ZkevmTestnet => 1442,
            Chain::BLAST => 81457,
            Chain::LOCAL => 31337,
        }
    }

    pub fn to_lowercase(&self) -> String {
        match self {
            Chain::ETH => "eth",
            Chain::GOERLI => "goerli",
            Chain::SEPOLIA => "sepolia",
            Chain::BSC => "bsc",
            Chain::CHAPEL => "chapel",
            Chain::POLYGON => "polygon",
            Chain::MUMBAI => "mumbai",
            Chain::FANTOM => "fantom",
            Chain::AVALANCHE => "avalanche",
            Chain::OPTIMISM => "optimism",
            Chain::ARBITRUM => "arbitrum",
            Chain::GNOSIS => "gnosis",
            Chain::BASE => "base",
            Chain::CELO => "celo",
            Chain::ZKEVM => "zkevm",
            Chain::ZkevmTestnet => "zkevm_testnet",
            Chain::BLAST => "blast",
            Chain::LOCAL => "local",
        }
        .to_string()
    }

    pub fn get_chain_rpc(&self) -> String {
        if let Ok(url) = env::var("ETH_RPC_URL") {
            return url;
        }
        match self {
            Chain::ETH => "https://eth.merkle.io",
            Chain::GOERLI => "https://rpc.ankr.com/eth_goerli",
            Chain::SEPOLIA => "https://rpc.ankr.com/eth_sepolia",
            Chain::BSC => "https://rpc.ankr.com/bsc",
            Chain::CHAPEL => "https://rpc.ankr.com/bsc_testnet_chapel",
            Chain::POLYGON => "https://polygon.llamarpc.com",
            Chain::MUMBAI => "https://rpc-mumbai.maticvigil.com/",
            Chain::FANTOM => "https://rpc.ankr.com/fantom",
            Chain::AVALANCHE => "https://rpc.ankr.com/avalanche",
            Chain::OPTIMISM => "https://rpc.ankr.com/optimism",
            Chain::ARBITRUM => "https://rpc.ankr.com/arbitrum",
            Chain::GNOSIS => "https://rpc.ankr.com/gnosis",
            Chain::BASE => "https://developer-access-mainnet.base.org",
            Chain::CELO => "https://rpc.ankr.com/celo",
            Chain::ZKEVM => "https://rpc.ankr.com/polygon_zkevm",
            Chain::ZkevmTestnet => "https://rpc.ankr.com/polygon_zkevm_testnet",
            Chain::BLAST => "https://rpc.ankr.com/blast",
            Chain::LOCAL => "http://localhost:8545",
        }
        .to_string()
    }

    pub fn get_chain_etherscan_base(&self) -> String {
        match self {
            Chain::ETH => "https://api.etherscan.io/api",
            Chain::GOERLI => "https://api-goerli.etherscan.io/api",
            Chain::SEPOLIA => "https://api-sepolia.etherscan.io/api",
            Chain::BSC => "https://api.bscscan.com/api",
            Chain::CHAPEL => "https://api-testnet.bscscan.com/api",
            Chain::POLYGON => "https://api.polygonscan.com/api",
            Chain::MUMBAI => "https://mumbai.polygonscan.com/api",
            Chain::FANTOM => "https://api.ftmscan.com/api",
            Chain::AVALANCHE => "https://api.snowtrace.io/api",
            Chain::OPTIMISM => "https://api-optimistic.etherscan.io/api",
            Chain::ARBITRUM => "https://api.arbiscan.io/api",
            Chain::GNOSIS => "https://api.gnosisscan.io/api",
            Chain::BASE => "https://api.basescan.org/api",
            Chain::CELO => "https://api.celoscan.io/api",
            Chain::ZKEVM => "https://api-zkevm.polygonscan.com/api",
            Chain::ZkevmTestnet => "https://api-testnet-zkevm.polygonscan.com/api",
            Chain::BLAST => "https://api.routescan.io/v2/network/mainnet/evm/81457/etherscan",
            Chain::LOCAL => "http://localhost:8080/abi/",
        }
        .to_string()
    }
}

#[derive(Clone, Default)]
pub struct OnChainConfig {
    pub endpoint_url: String,
    pub client: reqwest::blocking::Client,
    pub chain_id: u32,
    pub block_number: String,
    pub timestamp: Option<String>,
    pub coinbase: Option<String>,
    pub gaslimit: Option<String>,
    pub block_hash: Option<String>,

    pub etherscan_api_key: Vec<String>,
    pub etherscan_base: String,

    pub chain_name: String,

    balance_cache: HashMap<EVMAddress, EVMU256>,
    pair_cache: HashMap<EVMAddress, Vec<PairData>>,
    slot_cache: HashMap<(EVMAddress, EVMU256), EVMU256>,
    code_cache: HashMap<EVMAddress, String>,
    code_cache_analyzed: HashMap<EVMAddress, Bytecode>,
    price_cache: HashMap<EVMAddress, Option<(u32, u32)>>,
    abi_cache: HashMap<EVMAddress, Option<String>>,
    storage_dump_cache: HashMap<EVMAddress, Option<Arc<HashMap<EVMU256, EVMU256>>>>,
    uniswap_path_cache: HashMap<EVMAddress, TokenContext>,
    rpc_cache: FileSystemCache,
}

impl OnChainConfig {
    pub fn new(chain: Chain, block_number: u64) -> Self {
        Self::new_raw(
            chain.get_chain_rpc(),
            chain.get_chain_id(),
            block_number,
            chain.get_chain_etherscan_base(),
            chain.to_lowercase(),
        )
    }

    pub fn new_raw(
        endpoint_url: String,
        chain_id: u32,
        block_number: u64,
        etherscan_base: String,
        chain_name: String,
    ) -> Self {
        let mut s = Self {
            endpoint_url,
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(20))
                .build()
                .expect("build client failed"),
            chain_id,
            block_number: format!("0x{:x}", block_number),
            timestamp: None,
            coinbase: None,
            gaslimit: None,
            block_hash: None,
            etherscan_api_key: vec![],
            etherscan_base,
            chain_name,
            rpc_cache: FileSystemCache::new("./cache"),
            ..Default::default()
        };
        if block_number == 0 {
            s.set_latest_block_number();
        }
        s
    }

    pub fn set_latest_block_number(&mut self) {
        let resp = self._request("eth_blockNumber".to_string(), "[]".to_string());
        match resp {
            Some(resp) => {
                let block_number = resp.as_str().unwrap();
                self.block_number = block_number.to_string();
                let block_number =
                    EVMU256::from_str_radix(block_number.trim_start_matches("0x"), 16)
                        .unwrap()
                        .to_string();
                debug!("latest block number is {}", block_number);
            }
            None => panic!("fail to get latest block number"),
        }
    }

    fn _request(&self, method: String, params: String) -> Option<Value> {
        let data = format!(
            "{{\"jsonrpc\":\"2.0\", \"method\": \"{}\", \"params\": {}, \"id\": {}}}",
            method, params, self.chain_id
        );
        self.post(self.endpoint_url.clone(), data)
            .and_then(|resp| serde_json::from_str(&resp).ok())
            .and_then(|json: Value| json.get("result").cloned())
            .or_else(|| {
                error!("failed to fetch from {}", self.endpoint_url);
                None
            })
    }

    fn _request_with_id(&self, method: String, params: String, id: u8) -> Option<Value> {
        let data = format!(
            "{{\"jsonrpc\":\"2.0\", \"method\": \"{}\", \"params\": {}, \"id\": {}}}",
            method, params, id
        );
        self.post(self.endpoint_url.clone(), data)
            .and_then(|resp| serde_json::from_str(&resp).ok())
            .and_then(|json: Value| json.get("result").cloned())
            .or_else(|| {
                error!("failed to fetch from {}", self.endpoint_url);
                None
            })
    }

    fn post(&self, url: String, data: String) -> Option<String> {
        let mut hasher = DefaultHasher::new();
        let key = format!("post_{}_{}", url.as_str(), data.as_str());
        key.hash(&mut hasher);
        let hash = hasher.finish().to_string();
        if let Ok(t) = self.rpc_cache.load(hash.as_str()) {
            return Some(t);
        }
        match retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > 3 {
                return OperationResult::Err("did not succeed within 3 tries".to_string());
            }
            match self
                .client
                .post(url.to_string())
                .header("Content-Type", "application/json")
                .headers(get_header())
                .body(data.to_string())
                .send()
            {
                Ok(resp) => {
                    let text = resp.text();
                    match text {
                        Ok(t) => OperationResult::Ok(t),
                        Err(e) => {
                            error!("{:?}", e);
                            OperationResult::Retry("failed to parse response".to_string())
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                    OperationResult::Retry("failed to send request".to_string())
                }
            }
        }) {
            Ok(t) => {
                if !t.contains("error") {
                    self.rpc_cache.save(hash.as_str(), t.as_str()).unwrap();
                }
                Some(t)
            }
            Err(e) => {
                error!("Error: {}", e);
                None
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PairData {
    pub src: String,
    pub in_: i32,
    pub pair: String,
    pub in_token: String,
    pub next: String,
    pub interface: String,
    pub src_exact: String,
    pub initial_reserves_0: EVMU256,
    pub initial_reserves_1: EVMU256,
    pub decimals_0: u32,
    pub decimals_1: u32,
    pub token0: String,
    pub token1: String,
}

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("authority", "etherscan.io".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9,en;q=0.8".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Not?A_Brand\";v=\"8\", \"Chromium\";v=\"108\", \"Google Chrome\";v=\"108\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}