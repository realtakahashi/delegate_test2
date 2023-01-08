#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod source_contract {
    use destination_base::DestinationBaseRef;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SourceContract {
        /// Stores a single `bool` value on the storage.
        value: bool,
        destination_account_id:AccountId,
    }

    impl SourceContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, destination_account_id:AccountId) -> Self {
            Self { 
                value: init_value,
                destination_account_id: destination_account_id,
             }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
            self.value
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn execute_function(&mut self) {
            let mut instance: DestinationBaseRef = ink_env::call::FromAccountId::from_account_id(self.destination_account_id);
            instance.execute_function();
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let source_contract = SourceContract::default();
            assert_eq!(source_contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut source_contract = SourceContract::new(false);
            assert_eq!(source_contract.get(), false);
            source_contract.flip();
            assert_eq!(source_contract.get(), true);
        }
    }
}
