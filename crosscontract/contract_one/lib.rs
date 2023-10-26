#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
mod contract_one {
    use common_traits::contract_one::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct ContractOne {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl ContractOneTraits for ContractOne {
        #[ink(message)]
        fn flip(&mut self) {
            self.value = !self.value;
        }
        #[ink(message)]
        fn get(&self) -> bool {
            self.value
        }
    }

    impl ContractOne {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contract_one = ContractOne::default();
            assert_eq!(contract_one.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract_one = ContractOne::new(false);
            assert_eq!(contract_one.get(), false);
            contract_one.flip();
            assert_eq!(contract_one.get(), true);
        }
    }
}
