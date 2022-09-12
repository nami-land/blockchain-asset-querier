use super::defines::{SupportedContract, NetworkType};

#[derive(Debug, Default)]
struct AddressManager {}

impl AddressManager {
    fn get_contract_address(&self, contract_type: SupportedContract, network_type: NetworkType) -> &str {
        match contract_type {
            SupportedContract::BUSDTokenContract => {
                match network_type {
                    NetworkType::BSCMainNetwork => "0x2D6C8229E1e14F4D35037F977e5486EE1Bfa0190",
                    NetworkType::BSCTestNetwork => "0x2D6C8229E1e14F4D35037F977e5486EE1Bfa0190"
                }
            },
            SupportedContract::NECOTokenContract => {
                match network_type {
                    NetworkType::BSCMainNetwork => "0xd23891FC1A515A88C571064637502e3766819e2d",
                    NetworkType::BSCTestNetwork => "0xafA98d54481a9aE468AB21b9268609fF50795795"
                }
            },
            SupportedContract::NFISHTokenContract => {
                match network_type {
                    NetworkType::BSCMainNetwork => "0xa0c72B1F89531b6BD61C640d03Bd4507773C0cfC",
                    NetworkType::BSCTestNetwork => "0xa0c72B1F89531b6BD61C640d03Bd4507773C0cfC"
                }
            },
            SupportedContract::NecoNFTContract => {
                match network_type {
                    NetworkType::BSCMainNetwork => "0xEA5534Bac1291676595223579517D35Ad9C382eE",
                    NetworkType::BSCTestNetwork => "0xEB1C424A31490A9B141126838a3c625647f22BDc"
                }
            },
            SupportedContract::StakeNecoForFeeContract => {
                match network_type {
                    NetworkType::BSCMainNetwork => "0x8bfB9140658632239f8a1450955cB5bD7Ce586ED",
                    NetworkType::BSCTestNetwork => "0xa4329D80BE20813CbfeF5B2e593CA2893441E2dd"
                }
            },
        }
    }
}