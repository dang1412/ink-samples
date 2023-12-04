use openbrush::traits::Storage;
use crate::market::types::{ MarketData, MarketError };

#[openbrush::trait_definition]
pub trait MarketTrait:
    Storage<MarketData>
    // + Storage<ownable::Data>
    // + Storage<metadata::Data>
    // + PSP34Impl
    // + PSP34MetadataImpl
    // + psp34::extensions::metadata::Internal
    + Internal
{
    /// Mint
    #[ink(message)]
    fn mint(&mut self) -> Result<(), MarketError> {
        Ok(())
    }

    /// List NFT for sale
    #[ink(message)]
    fn list_for_sale(&mut self, nft_id: Id, price: Balance) -> Result<(), MarketError> {
        Ok(())
    }

    /// Buy the listed NFT
    #[ink(message, payable)]
    fn buy(&mut self, nft_id: Id) -> Result<(), MarketError> {
        Ok(())
    }

    /// Get list of listed NFT
    #[ink(message, payable)]
    fn get_list(&self, from: u32, to: u32) -> Vec<Id> {
        Vec::default()
    }
}

pub trait Internal: Storage<MarketData> {

}