use super::defines::{Error, NetworkType};
use crate::common::defines::SupportedContractType;
use ethers_core::types::Address;

// get contract address by contract type and network type
pub fn get_contract_address(
    contract_type: SupportedContractType,
    network_type: NetworkType,
) -> Result<Address, Error> {
    if network_type == NetworkType::EthereumMainnet {
        return Err("ethereum mainnet is not supported.".into());
    }

    if network_type == NetworkType::GoerliTestnet {
        return match contract_type {
            SupportedContractType::NamiLandGameItemNFT => {
                Ok("0x5FaB721a3fa13c0219EB24C121f9F6482f64f274".parse()?)
            }
            _ => Err("contract type is not supported.".into()),
        };
    }

    Err("network is not supported.".into())
}
