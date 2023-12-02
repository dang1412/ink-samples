#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract_two {
    use common_traits::contract_one::ContractOneRef;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct ContractTwo {
        addr_one: AccountId
    }

    impl ContractTwo {
        /// Constructor that initializes empty struct
        #[ink(constructor)]
        pub fn new(addr_one: AccountId) -> Self {
            Self { addr_one }
        }

        /// Cross-contract call.
        /// Simply returns the current value of contract_one `bool`.
        #[ink(message)]
        pub fn other_get(&self) -> bool {
            ContractOneRef::get(&self.addr_one)
        }

        /// Cross-contract call
        /// This one flips the value of the contract_one stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn other_flip(&mut self, addr: AccountId) {
            ContractOneRef::flip(&addr);
        }
    }
}
