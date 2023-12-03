use openbrush::traits::{
    AccountId,
    Balance,
    BlockNumber,
};

use openbrush::storage::{
    Mapping,
    MultiMapping,
};

use ink::storage::traits::StorageLayout;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct MarketData {
    mint_price: Balance,
    nft_prices: Mapping<Id, Balance>,
    sale_items: Mapping<u32, Id>,
    sale_count: u32,
}

/// The market error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
    /// Not owner of the NFT
    NotOwner,
    /// Pay not enough money
    NotEnoughPay,
}
