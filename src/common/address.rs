use super::defines::{ContractType, Error, NetworkType};
use crate::ADDRESS_MANAGER;
use ethers_core::types::Address;

pub fn get_contract_address(
    contract_type: &ContractType,
    network_type: &NetworkType,
) -> Result<Address, Error> {
    match network_type {
        NetworkType::BSCMainNetwork => match contract_type {
            ContractType::BUSD => Ok(ADDRESS_MANAGER
                .get()
                .expect("no busd config")
                .bsc_main
                .busd
                .parse::<Address>()?),
            ContractType::NFISH => Ok(ADDRESS_MANAGER
                .get()
                .expect("no nfish config")
                .bsc_main
                .nfish
                .parse::<Address>()?),
            ContractType::NECO => Ok(ADDRESS_MANAGER
                .get()
                .expect("no neco config")
                .bsc_main
                .neco
                .parse::<Address>()?),
            ContractType::NecoNFT => Ok(ADDRESS_MANAGER
                .get()
                .expect("no neco nft config")
                .bsc_main
                .neco_nft
                .parse::<Address>()?),
            ContractType::StakeNecoForFee => Ok(ADDRESS_MANAGER
                .get()
                .expect("no stake neco for fee config")
                .bsc_main
                .stake_neco_for_fee
                .parse::<Address>()?),
        },
        NetworkType::BSCTestNetwork => match contract_type {
            ContractType::BUSD => Ok(ADDRESS_MANAGER
                .get()
                .expect("no busd config")
                .bsc_test
                .busd
                .parse::<Address>()?),
            ContractType::NFISH => Ok(ADDRESS_MANAGER
                .get()
                .expect("no nfish config")
                .bsc_test
                .nfish
                .parse::<Address>()?),
            ContractType::NECO => Ok(ADDRESS_MANAGER
                .get()
                .expect("no neco config")
                .bsc_test
                .neco
                .parse::<Address>()?),
            ContractType::NecoNFT => Ok(ADDRESS_MANAGER
                .get()
                .expect("no neco nft config")
                .bsc_test
                .neco_nft
                .parse::<Address>()?),
            ContractType::StakeNecoForFee => Ok(ADDRESS_MANAGER
                .get()
                .expect("no stake neco for fee config")
                .bsc_test
                .stake_neco_for_fee
                .parse::<Address>()?),
        },
    }
}
