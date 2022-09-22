use super::defines::{ContractType, Error, NetworkType};
use ethers_core::types::Address;

pub fn get_contract_address(
    contract_type: &ContractType,
    network_type: &NetworkType,
) -> Result<Address, Error> {
    match contract_type {
        ContractType::BUSD => match network_type {
            NetworkType::BSCMainNetwork => {
                Ok("0x2D6C8229E1e14F4D35037F977e5486EE1Bfa0190".parse::<Address>()?)
            }
            NetworkType::BSCTestNetwork => {
                Ok("0x2D6C8229E1e14F4D35037F977e5486EE1Bfa0190".parse::<Address>()?)
            }
        },
        ContractType::NECO => match network_type {
            NetworkType::BSCMainNetwork => {
                Ok("0xd23891FC1A515A88C571064637502e3766819e2d".parse::<Address>()?)
            }
            NetworkType::BSCTestNetwork => {
                Ok("0xafA98d54481a9aE468AB21b9268609fF50795795".parse::<Address>()?)
            }
        },
        ContractType::NFISH => match network_type {
            NetworkType::BSCMainNetwork => {
                Ok("0xa0c72B1F89531b6BD61C640d03Bd4507773C0cfC".parse::<Address>()?)
            }
            NetworkType::BSCTestNetwork => {
                Ok("0xa0c72B1F89531b6BD61C640d03Bd4507773C0cfC".parse::<Address>()?)
            }
        },
        ContractType::NecoNFT => match network_type {
            NetworkType::BSCMainNetwork => {
                Ok("0xEA5534Bac1291676595223579517D35Ad9C382eE".parse::<Address>()?)
            }
            NetworkType::BSCTestNetwork => {
                Ok("0xEB1C424A31490A9B141126838a3c625647f22BDc".parse::<Address>()?)
            }
        },
        ContractType::StakeNecoForFee => match network_type {
            NetworkType::BSCMainNetwork => {
                Ok("0x8bfB9140658632239f8a1450955cB5bD7Ce586ED".parse::<Address>()?)
            }
            NetworkType::BSCTestNetwork => {
                Ok("0xa4329D80BE20813CbfeF5B2e593CA2893441E2dd".parse::<Address>()?)
            }
        },
    }
}
