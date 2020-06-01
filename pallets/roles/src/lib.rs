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

// TODO: add enum or struct for roles here.

// This pallet's errors.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// The role does not exist, so it cannot be revoked
		NoSuchRole,
	}
}

// This is the pallet's events.
decl_event! (
    pub enum Event<T> 
    where 
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Event emitted when a role has been claimed.
        ClaimRole(AccountId, u8),
        /// Event emitted when a role is revoked by the owner.
        //RevokedRole(AccountId, u8),
    }
);

// This is the pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as RoleModule {
        /// The storage item for our roles.
        /// It maps a role to the user who made claimed the role and when they made it.
        Roles get(fn roles): map hasher(blake2_128_concat) T::AccountId => u8;
    }
}


// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        type Error = Error<T>;
        
        // A default function for depositing events
        fn deposit_event() = default;

        /// Allow a user to take a role
        #[weight = 10_000]
        fn claim_role(origin, role: u8) {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // Store the proof with the sender and the current block number
            Roles::<T>::insert(&sender, role);

            // Emit an event that the role was claimed
            Self::deposit_event(RawEvent::ClaimRole(sender, role));
        }
    }
}

