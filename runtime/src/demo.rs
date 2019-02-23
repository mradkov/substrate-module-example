/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs
use parity_codec::Encode;

use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};

// Enables access to account balances and interacting with signed messages
use {balances, system::{self, ensure_signed}};

// Enables us to do hashing
use runtime_primitives::traits::Hash;

/// The module's configuration trait.
pub trait Trait: balances::Trait {
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn play(origin) -> Result {
			// Logic for playing the game

			//Ensure we have a signed message and derive the sender's account id from the signature
			let sender = ensure_signed(origin)?;

			// Here we grab the payment and put it into a local variable.
			// We are able to use Self::payment() because we defined it in our decl_storage! macro below
			// If there is no payment, exit with an error message
			let payment = Self::payment().ok_or("Must have payment amount set")?;

			// First we decrease the balances of the sender by the payment amount using the balances module
			<balances::Module<T>>::decrease_free_balance(&sender, payment)?;

			// Then we flip a coin generating a random seed
			// We pass the seed with our sender's account id into a hash algorithm
			// Then we check if the first byte of the hash is less than 128
			if (<system::Module<T>>::random_seed(), &sender)
			.using_encoded(<T as system::Trait>::Hashing::hash)
			.using_encoded(|e| e[0] < 128)
			{
				// If the sender wins the coin flip, we increase the sender's balance by the pot amount
				// `::take()` will also remove the pot amount from storage, which by default will give it a value of 0
				<balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
			}

			// No matter the outcome we will add the original sender's payment back into the pot
			<Pot<T>>::mutate(|pot| *pot += payment);

			Ok(())
		}

		fn set_payment(_origin, value: T::Balance) -> Result {
			// Logic for setting the game payment

			// If the payment has not been set
			if Self::payment().is_none() {
				// we will set it to the value we passed in
				<Payment<T>>::put(value);

				// We will also put that initial value into the pot for someone to win
				<Pot<T>>::put(value);
			}

			Ok(())
		}
	}
}

/// This module's storage items.
decl_storage! {
	trait Storage for Module<T: Trait> as Demo {
		Payment get(payment): Option<T::Balance>;
		Pot get(pot): T::Balance;
	}
}