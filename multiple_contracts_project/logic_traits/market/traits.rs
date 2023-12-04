use ink::prelude::{
    string::String,
    vec::Vec,
};

use openbrush::{
    traits::{
        AccountId,
        Balance,
    },
    contracts::psp34::PSP34Error,
};

#[openbrush::wrapper]
pub type MarketRef = dyn MarketTraitRef;

/// Market method definitions.
/// Actually only methods used by other contract (cross-contract call) are needed.
#[openbrush::trait_definition]
pub trait MarketTraitRef {
    /// Mint
    #[ink(message)]
    fn mint(&mut self) -> Result<(), MarketError>;

    /// List NFT for sale
    #[ink(message)]
    fn list_for_sale(&mut self, nft_id: Id, price: Balance) -> Result<(), MarketError>;

    /// Buy the listed NFT
    #[ink(message, payable)]
    fn buy(&mut self, nft_id: Id) -> Result<(), MarketError>;

    /// Get list of listed NFT
    #[ink(message, payable)]
    fn get_list(&self, from: u32, to: u32) -> Vec<Id>;
}