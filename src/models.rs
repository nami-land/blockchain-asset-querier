use crate::common::defines::NetworkType;

#[derive(Debug, Clone)]
pub struct NecoNFTTrait {
    trait_type: String,
    value: String,
}

#[derive(Debug, Clone, Default)]
pub struct NecoNFTMetadata {
    id: String,
    name: String,
    description: String,
    nft_type1: String,
    nft_type2: String,
    animation_url: String,
    image_url: String,
    external_url: String,
    attributes: Vec<NecoNFTTrait>,
}

pub struct OwnershipItem {
    nft_id: String,
    amount: String,
    nft_metadata: NecoNFTMetadata,
}

pub struct NecoNFTOwnership {
    pub public_address: String,
    pub network: NetworkType,
    pub contract_address: String,
    pub ownerships: Vec<OwnershipItem>,
}
