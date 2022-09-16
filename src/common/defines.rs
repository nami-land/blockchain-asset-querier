use ethers::abi::Uint;
use std::fmt;
use std::fmt::Formatter;

pub type Error = Box<dyn std::error::Error + Sync + Send>;

// define a enum for the blockchain type.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NetworkType {
    BSCMainNetwork,
    BSCTestNetwork,
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NetworkType::BSCMainNetwork => write!(f, "bsc_main_network"),
            NetworkType::BSCTestNetwork => write!(f, "bsc_test_network"),
        }
    }
}

#[derive(Debug)]
pub enum GameClient {
    NecoFishing,
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for GameClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameClient::NecoFishing => write!(f, "neco_fishing"),
        }
    }
}

#[derive(Debug)]
pub enum SupportedERC20Token {
    NECO,
    NFISH,
    BUSD,
}

// implement the Display trait to convert enum to a string.
impl fmt::Display for SupportedERC20Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// All supported nft ids for neco fishing game.
const NECO_FISHING_NFT_IDS: [u32; 57] = [
    10001, 10002, 10003, 10004, 10005, 10006, 10007, 11001, 11002, 11003, 11004, 11005, 11006,
    11007, 12001, 12002, 12003, 13001, 13002, 13003, 13004, 13005, 13006, 14001, 14002, 15001,
    15002, 15003, 15004, 15005, 15006, 15007, 15008, 15009, 15010, 15011, 15012, 15013, 15014,
    15015, 15016, 16001, 16002, 16003, 16004, 16005, 16006, 16007, 16008, 16009, 16010, 16011,
    16012, 16013, 16014, 16015, 16016,
];

pub const BSC_MAIN_NETWORK_RPC: &str = "https://bsc-dataseed.binance.org";
pub const BSC_TEST_NETWORK_RPC: &str = "https://data-seed-prebsc-1-s3.binance.org:8545/";

#[derive(Debug)]
pub enum SupportedContract {
    NECOTokenContract,
    NFISHTokenContract,
    BUSDTokenContract,
    NecoNFTContract,
    StakeNecoForFeeContract,
}
