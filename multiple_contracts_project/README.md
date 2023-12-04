# Multiple Contracts Project

- PSP34 with market feature
- Other contracts

Create workspace `Cargo.toml`

```toml
[workspace]
members = [
    "psp34_market", # contract PSP34 with market feature
    # "other_contract",
    "logic_traits", # main implementation of all contracts
]
resolver = "2"

[workspace.package]
authors = ["[your_name] <[your_email]>"]
edition = "2021"
repository = "https://github.com/dang1412/ink-samples/multiple_contracts_project/"
```

## PSP34 Market

Create contract

```sh
cargo contract new psp34_market
```

Update `Cargo.toml` with Openbrush crate as dependency

```toml
openbrush = { tag = "4.0.0-beta", git = "https://github.com/Brushfam/openbrush-contracts", default-features = false, features = ["psp34", "ownable"] }

[features]
default = ["std"]
std = [
    # ...
    "openbrush/std",
]
```

Update contract `lib.rs` with openbrush PSP34

```rs
// Apply imlementation of these feature: PSP34, metadata, enumerable, ownable
#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
mod psp34_market {
// ...

    // Define data used by the above implementation
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
}
```

Build contract should be successful

```sh
cd psp34_market
cargo contract build
```

By doing this, we have a contract with basic features of an PSP34 standard NFT implemented by Openbrush.

- NFT
- NFT with metadata
- NFT enumerable
- Check Ownable

Next, we will implement our own features of this NFT.

## Logic crate

`logic_traits` is a common module contains logic slices of all contracts in the project. For example it contains

- LogicSliceA (used by contract_one)
- LogicSliceB (used by contract_one)
- LogicSlideC (used by contract_two)
- LogicSlideD (used by both contracts, the `upgradable` trait in this case)

The reason all the logic defined (and implemented as well for convenient) in a common module is for cross-contract call betweens contracts in the project.

Each logic slice basically contains

- `types.rs`: define data, error.
- `traits.rs`: define trait functions that can be used by other contracts by cross-contract call.
- `impls.rs`: implement all message functions, include but not limited to functions defined in `traits.rs`.

In this tutorial we are going to implement PSP34 very basic market feature.

### Types

Data

```rs
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct MarketData {
    mint_price: Balance,
    nft_prices: Mapping<Id, Balance>,

    // Array of nfts for sale (use Mapping instead of Vec)
    sale_items: Mapping<u32, Id>,
    sale_count: u32,
}
```

Error

```rs
/// The market error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
    /// Not owner of the NFT
    NotOwner,
    /// Pay not enough money
    NotEnoughPay,
    /// Already minted
    AlreadyMinted,
}
```

### Traits

```rs
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
```

### Implementation

Notice that by defining the above, we already have these traits implemented for the contract struct `psp34_market`.
All the below types and traits can be imported from Openbrush.

```rs
// Storage - The data type imported from Openbrush
Storage<psp34::Data>
Storage<metadata::Data>
Storage<ownable::Data>

// Implementation
PSP34Impl
PSP34MetadataImpl
psp34::extensions::metadata::Internal
//...some more
```

Then we can use it as we want in our own logic implementation.

Some tips

```rs
// To access the data struct defined in 'metadata::Data'
self.data::<metadata::Data>()
// similar for own defined data
self.data::<MarketData>()

// call function '_owner_of' defined in trait 'PSP34Impl'
self._owner_of(Id:u16(100))
// or
psp34::Internal::_owner_of(self, &Id::U16(100))
```

Usually we separate the implementaion into 2 parts

- Main public message implementation, for short and clear about ink message methods
- Internal implemetaion used by main, contains complex or reusable logic

Also, we list all dependency traits when define our trait implementation

```rs
// Openbrush macro
#[openbrush::trait_definition]
// Define main trait with all dependencies
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

// Define internal trait with all dependencies
pub trait Internal:
    Storage<MarketData>
    // + Storage<MarketData>
    // + PSP34Impl
    // + psp34::extensions::metadata::Internal
{
    //...internal methods
}

```

### Apply in contract

In our main contract,

`Cargo.toml`

Add `logic_traits` crate as dependency.

`lib.rs`

Import the traits definition from `logic_traits` crate

Apply trait `MarketTrait` and `Internal` using default implementation

```rs
    // Market logic
    impl MarketTrait for Psp34Market {}
    impl MarketInternal for Psp34Market {}
```

## Events

## Upgradable
