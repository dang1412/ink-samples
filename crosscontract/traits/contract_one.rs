#[openbrush::wrapper]
pub type ContractOneRef = dyn ContractOneTraits;

#[openbrush::trait_definition]
pub trait ContractOneTraits {
    #[ink(message)]
    fn flip(&mut self);
    #[ink(message)]
    fn get(&self) -> bool;
}
