#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, 
    decl_event,
    dispatch::DispatchResult, 
    StorageMap
};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;
use codec::{Encode, Decode};
use rstd::prelude::*;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

//type Hash = H256;
type Hash = sp_core::H256;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Asset<Hash> {
    id: Hash,
    name: Vec<u8>,
    ref_system: Vec<u8>,
    ref_id: Vec<u8>,
}

// This is the pallet's events.
decl_event! (
    pub enum Event<T> 
    where 
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Event emitted when a asset has been created.
        AssetCreated(AccountId, Vec<u8>, Vec<u8>, Vec<u8>, Hash),
    }
);

// This is the pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as AssethandlerModule {
        // It maps a asset to the user who created the asset and when the asset was created.
        Assets: map hasher(blake2_128_concat) Hash => (T::AccountId, T::BlockNumber);
    }
}


// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // cd type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

        /// Let a user create a new asset
        #[weight = 10_000]
        fn create_claim(origin, name: Vec<u8>, ref_sys: Vec<u8>, ref_id: Vec<u8>, id: Hash) -> DispatchResult {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not been claimed yet or error with the message
            // ensure!(!Assets::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();            

            let _asset = Asset {
                id: id.clone(),
                name: name.clone(),
                ref_system: ref_sys.clone(),
                ref_id: ref_id.clone(),
            };

            // Store the asset with the sender and the current block number
            Assets::<T>::insert(id, (&sender, current_block));

            // Emit an event that the claim was created
            Self::deposit_event(RawEvent::AssetCreated(sender, name, ref_sys, ref_id, id));

            Ok(())
        }
    }
}

