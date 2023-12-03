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
    #[ink(message)]
    fn mint(&mut self);

    #[ink(message)]
    fn list_for_sale(&mut self, nft_id: Id, price: Balance);
}