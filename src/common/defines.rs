use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use utoipa::ToSchema;

pub type Error = Box<dyn std::error::Error + Sync + Send>;

// define a enum for the blockchain type.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub enum NetworkType {
    EthereumMainnet,
    GoerliTestnet,
    BSCMainNetwork,
    BSCTestNetwork,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::EthereumMainnet
    }
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NetworkType::EthereumMainnet => write!(f, "EthereumMainnet"),
            NetworkType::GoerliTestnet => write!(f, "GoerliTestnet"),
            NetworkType::BSCMainNetwork => write!(f, "bsc_main_network"),
            NetworkType::BSCTestNetwork => write!(f, "bsc_test_network"),
        }
    }
}

#[derive(Debug)]
pub enum GameClient {
    NamiLand,
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for GameClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameClient::NamiLand => write!(f, "namiland"),
        }
    }
}

#[derive(Debug)]
pub enum SupportedERC20Token {
    NAMIX,
    FISHX,
    ETH,
    USDC,
    USDT,
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for SupportedERC20Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// All supported nft ids for NamiLand.
pub const NAMILAND_GAME_ITEM_NFT_IDS: [i32; 48] = [
    10001, 10002, 10003, 10004, 10005, 10006, 10007, 11001, 11002, 11003, 11004, 11005, 11006,
    11007, 12001, 12002, 13001, 13002, 13003, 13004, 13005, 13006, 13007, 13008, 13009, 13010,
    13011, 13012, 13013, 13014, 13015, 13016, 14001, 14002, 14003, 14004, 14005, 14006, 14007,
    14008, 14009, 14010, 14011, 14012, 14013, 14014, 14015, 14016,
];

pub const ETHEREUM_MAINNET_NETWORK_RPC: &str =
    "https://mainnet.infura.io/v3/89f31b5b62a44ed68b4f73c35be6c81f";
pub const GOERLI_TESTNET_NETWORK_RPC: &str =
    "https://goerli.infura.io/v3/89f31b5b62a44ed68b4f73c35be6c81f";
pub const BSC_MAIN_NETWORK_RPC: &str = "https://bsc-dataseed.binance.org/";
pub const BSC_TEST_NETWORK_RPC: &str = "https://data-seed-prebsc-1-s2.binance.org:8545/";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SupportedContractType {
    NAMIX,
    FISHX,
    NamiLandGameItemNFT,
}
