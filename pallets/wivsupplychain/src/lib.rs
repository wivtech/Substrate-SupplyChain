#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::prelude::*;
use frame_support::{decl_module, decl_event, decl_storage, ensure, decl_error,dispatch};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// pallet parameters definition
pub trait Trait: frame_system::Trait {	
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The runtime storage definition
decl_storage! {
	trait Store for Module<T: Trait> as WivSupplyChain {
		Asset: map hasher(twox_64_concat) T::AccountId => Option<Vec<u8>>;
	}
}

// Events generated from Wiv-Supplychain pallet
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
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		/// New asset storage
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
		/// Transfer of an asset
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
		/// Transfer of an asset
		#[weight = 500_000]
		pub fn remove_asset(origin, asset: Vec<u8>) -> dispatch::DispatchResult {
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
	}
}
