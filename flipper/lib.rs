#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
        /// Stores a single `u128` value representing the total balance.
        total_balance: u128,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value` and the `total_balance` to `init_balance`.
        #[ink(constructor)]
        pub fn new(init_value: bool, init_balance: u128) -> Self {
            Self { value: init_value, total_balance: init_balance }
        }

        /// Constructor that initializes the `bool` value to `false` and `total_balance` to 0.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), 0)
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true` to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Returns the current value of `total_balance`.
        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            self.total_balance
        }

         /// Change total balance
        #[ink(message)]
        pub fn set_balance(&mut self, new_balance: u128) {
        self.total_balance = new_balance;
}

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
            assert_eq!(flipper.get_balance(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false, 100);
            assert_eq!(flipper.get(), false);
            assert_eq!(flipper.get_balance(), 100);
            flipper.flip();
            assert_eq!(flipper.get(), true);
            assert_eq!(flipper.get_balance(), 100);
        }
    }
}
