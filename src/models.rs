use crate::common::defines::NetworkType;

pub struct NecoNFTTrait {
    trait_type: String,
    value: String,
}

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
    public_address: String,
    network: NetworkType,
    contract_address: String,
    ownerships: Vec<OwnershipItem>,
}
