#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::destination_advanced::{DestinationAdvanced,DestinationAdvancedRef};

#[ink::contract]
mod destination_advanced {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DestinationAdvanced {
        /// Stores a single `bool` value on the storage.
        value: bool,
        value2: u8,
    }

    impl DestinationAdvanced {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
                value2: 0,
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
        pub fn inc(&mut self) {
            self.value2 += 1;
            ink_env::debug_println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$ inc executed.");
        }

        #[ink(message)]
        pub fn get_value2(&self) -> u8 {
            self.value2
        }

        #[ink(message)]
        pub fn execute_function(&mut self) {
            ink_env::debug_println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$ inc called.");
            self.inc();
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
            let destination_advanced = DestinationAdvanced::default();
            assert_eq!(destination_advanced.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut destination_advanced = DestinationAdvanced::new(false);
            assert_eq!(destination_advanced.get(), false);
            destination_advanced.flip();
            assert_eq!(destination_advanced.get(), true);
        }
    }
}
