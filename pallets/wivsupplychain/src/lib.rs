#![cfg_attr(not(feature = "std"), no_std)]

//use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
/*use frame_system::ensure_signed;
use frame_support::{
	decl_module, decl_event, decl_storage, ensure, decl_error,dispatch,
	traits::{Currency, EnsureOrigin, ReservableCurrency, OnUnbalanced, Get},
};*/
extern crate alloc;
use sp_std::prelude::*;
use frame_support::{
	decl_module, decl_event, decl_storage, ensure, decl_error,dispatch,
	traits::{Currency, EnsureOrigin, ReservableCurrency, OnUnbalanced, Get},
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as WivSupplyChain {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		Asset: map hasher(twox_64_concat) T::AccountId => Option<Vec<u8>>;

	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// New asset has been stored (base64 encoding + json)
		/// [Asset, AccountId]
		NewAssetStored(Vec<u8>, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
		/// asset data is too short
		TooShort,
		/// asset data is too long
		TooLong,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// new asset storage
		#[weight = 500_000]
		pub fn new_asset(origin, asset: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;
			ensure!(asset.len() >= 16, Error::<T>::TooShort); //check minimum length
			ensure!(asset.len() <= 8192, Error::<T>::TooLong);  // check maximum length
			// Update storage.
			let assetstorage=asset.clone();
			<Asset<T>>::insert(&sender, assetstorage);
			// Emit an event
			Self::deposit_event(RawEvent::NewAssetStored(asset, sender));
			// Return a successful DispatchResult
			Ok(())
		}
		/// transfer asset
		#[weight = 500_000]
		pub fn transfer_asset(origin, asset: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;
			ensure!(asset.len() >= 16, Error::<T>::TooShort); //check minimum length
			ensure!(asset.len() <= 8192, Error::<T>::TooLong);  // check maximum length
			// Update storage.
			let assetstorage=asset.clone();
			<Asset<T>>::insert(&sender, assetstorage);
			// Emit an event
			Self::deposit_event(RawEvent::NewAssetStored(asset, sender));
			// Return a successful DispatchResult
			Ok(())
		}
		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn get_asset(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
