#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
mod psp34_market {
    use openbrush::{
        contracts::{
            ownable,
            psp34,
            psp34::{
                extensions::{
                    enumerable,
                    metadata,
                },
                PSP34Impl,
            },
        },
        traits::Storage,
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Psp34Market {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl Psp34Market {
        /// Constructor that initializes some values.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            // set some init values

            // return instance
            instance
        }
    }
}
