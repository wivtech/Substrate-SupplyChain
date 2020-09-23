#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc721 {
    use ink_core::storage;
    use scale::{
        Decode,
        Encode,
    };
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;

    /// A token ID.
    // pub type TokenId = u32;

    #[ink(storage)]
    struct Erc721 {
        /// Contract version
        version: storage::Value<u8>,
        /// Owner, superadmin
        owner: storage::Value<AccountId>,

        /// Mapping from token to owner.
        token_owner: storage::HashMap<u32, AccountId>,        // u32 - TokenId
        /// Mapping from token to approvals users.
        token_approvals: storage::HashMap<u32, AccountId>,         // u32 - TokenId
        /// Mapping from owner to number of owned token.
        owned_tokens_count: storage::HashMap<AccountId, u32>,
        /// Mapping from owner to operator approvals.
        operator_approvals: storage::HashMap<(AccountId, AccountId), bool>,

        /// List of total tokens
        tokens: storage::Vec<u32>,

        /// Mapping from account to metadata
        account_metadata: storage::HashMap<AccountId, Vec<u8>>,
        /// Supply chain nodes
        supply_chain_data: storage::HashMap<AccountId, Vec<u8>>,
        /// Roles data v1
        roles_data: storage::HashMap<AccountId, Vec<u8>>,
        /// Token metadata
        token_metadata: storage::HashMap<u32, Vec<u8>>
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotRemove,
        CannotFetchValue,
        NotAllowed,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: u32, // u32 - TokenId
    }

    /// Event emited when a token approve occurs.
    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: u32,        // u32 - TokenId
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    impl Erc721 {
        /// Creates a new ERC721 token contract.
        #[ink(constructor)]
        fn new(&mut self) {
          self.version.set(3);
          self.owner.set(self.env().caller());
        }

        /// ========================================================================================================================
        /// ========================================================================================================================
        /// ========================================================================================================================
        /// Get version of contract
        #[ink(message)]
        fn version(&self) -> u8 {
          *self.version
        }

        /// Check if owner
        #[ink(message)]
        fn is_contract_owner(&self, address: AccountId) -> bool {
          self.contract_owner() == address
        }

        /// Get owner of contract
        #[ink(message)]
        fn contract_owner(&self) -> AccountId {
          *self.owner
        }

        /// Returns the balance of the owner.
        ///
        /// This represents the amount of unique tokens the owner has.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        /// Returns the owner of the token.
        #[ink(message)]
        fn owner_of(&self, id: u32) -> Option<AccountId> {         // u32 - TokenId
            self.token_owner.get(&id).cloned()
        }

        /// Returns the approved account ID for this token if any.
        #[ink(message)]
        fn get_approved(&self, id: u32) -> Option<AccountId> {         // u32 - TokenId
            self.token_approvals.get(&id).cloned()
        }

        /// Returns `true` if the operator is approved by the owner.
        #[ink(message)]
        fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.approved_for_all(owner, operator)
        }

        /// Approves or disapproves the operator for all tokens of the caller.
        #[ink(message)]
        fn set_approval_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            self.approve_for_all(to, approved)?;
            Ok(())
        }

        /// Approves the account to transfer the specified token on behalf of the caller.
        #[ink(message)]
        fn approve(&mut self, to: AccountId, id: u32) -> Result<(), Error> {         // u32 - TokenId
            self.approve_for(&to, id)?;
            Ok(())
        }

        /// Transfers the token from the caller to the given destination.
        #[ink(message)]
        fn transfer(&mut self, destination: AccountId, id: u32) -> Result<(), Error> {         // u32 - TokenId
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &destination, id)?;
            Ok(())
        }

        /// Transfer approved or owned token.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: u32,         // u32 - TokenId
        ) -> Result<(), Error> {
            self.transfer_token_from(&from, &to, id)?;
            Ok(())
        }

        /// Creates a new token.
        #[ink(message)]
        fn mint(&mut self, id: u32) -> Result<(), Error> {         // u32 - TokenId
            let caller = self.env().caller();
            self.add_token_to(&caller, id)?;
            self.tokens.push(id);
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                id,
            });
            Ok(())
        }

        /// Deletes an existing token. Only the owner can burn the token.
        #[ink(message)]
        fn burn(&mut self, id: u32) -> Result<(), Error> {         // u32 - TokenId
            let caller = self.env().caller();
            if self.token_owner.get(&id) != Some(&caller) {
                return Err(Error::NotOwner)
            };
            self.remove_token_from(&caller, id)?;
            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(AccountId::from([0x0; 32])),
                id,
            });
            Ok(())
        }

        /// Account metadata - Get
        #[ink(message)]
        fn account_metadata_of(&self, owner: AccountId) -> Vec<u8> {
            self.account_metadata_of_or_empty(&owner)
        }

        #[ink(message)]
        fn account_metadata_of_as_string(&self, owner: AccountId) -> String {
            let value = self.account_metadata_of_or_empty(&owner);
            String::from_utf8(value.to_vec()).unwrap()
        }

        /// Account metadata - Set
        #[ink(message)]
        fn set_account_metadata_of(&mut self, owner: AccountId, metadata: Vec<u8>) -> bool {
            if self.is_contract_owner(self.env().caller()) || owner == self.env().caller() {
                self.account_metadata.insert(owner, metadata);
                return true;
            } else {
                return false;
            };
        }

        /// Supply chain - Get
        #[ink(message)]
        fn supply_chain(&self) -> Vec<u8> {
            self.supply_chain_data_of_or_empty(&*self.owner)
        }

        /// Supply chain - Get
        #[ink(message)]
        fn supply_chain_as_string(&self) -> String {
            let value = self.supply_chain_data_of_or_empty(&*self.owner);
            String::from_utf8(value.to_vec()).unwrap()
        }

        /// Supply chain - Set
        #[ink(message)]
        fn set_supply_chain(&mut self, data: Vec<u8>) -> bool {
            self.supply_chain_data.insert(*self.owner, data);
            true
        }

        /// Roles - Get
        #[ink(message)]
        fn roles(&self) -> Vec<u8> {
            self.roles_data_of_or_empty(&*self.owner)
        }

        /// Roles - Get
        #[ink(message)]
        fn roles_as_string(&self) -> String {
            let value = self.roles_data_of_or_empty(&*self.owner);
            String::from_utf8(value.to_vec()).unwrap()
        }

        /// Roles - Set
        #[ink(message)]
        fn set_roles(&mut self, data: Vec<u8>) -> bool {
            self.roles_data.insert(*self.owner, data);
            true
        }

        /// Tokens - Get all tokens
        #[ink(message)]
        fn list_all_tokens(&self) -> Vec<u32> {
            let mut clone: Vec<u32> = Vec::new();

            for x in self.tokens.iter() {
              clone.push(*x)
            }

            clone
        }

        /// Token metadata - Get
        #[ink(message)]
        fn token_metadata_of(&self, token: u32) -> Vec<u8> {
            self.token_metadata_of_or_empty(&token)
        }

        #[ink(message)]
        fn token_metadata_of_as_string(&self, token: u32) -> String {
            let value = self.token_metadata_of_or_empty(&token);
            String::from_utf8(value.to_vec()).unwrap()
        }

        /// Token metadata - Set
        #[ink(message)]
        fn set_token_metadata_of(&mut self, token: u32, metadata: Vec<u8>) -> bool {
            if !self.exists(token) {
                return false
            };

            self.token_metadata.insert(token, metadata);
            true
        }

        /// ========================================================================================================================
        /// ========================================================================================================================
        /// ========================================================================================================================
        /// Internal functions
        /// Transfers token `id` `from` the sender to the `to` AccountId.
        fn transfer_token_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: u32,         // u32 - TokenId
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            };
            if !self.approved_or_owner(Some(caller), id) {
                return Err(Error::NotApproved)
            };
            self.clear_approval(id)?;
            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });
            Ok(())
        }

        /// Removes token `id` from the owner.
        fn remove_token_from(
            &mut self,
            from: &AccountId,
            id: u32,         // u32 - TokenId
        ) -> Result<(), Error> {
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            }
            self.decrease_counter_of(from)?;
            self.token_owner.remove(&id).ok_or(Error::CannotRemove)?;
            Ok(())
        }

        /// Adds the token `id` to the `to` AccountID.
        fn add_token_to(&mut self, to: &AccountId, id: u32) -> Result<(), Error> {         // u32 - TokenId
            if self.exists(id) {
                return Err(Error::TokenExists)
            };
            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };
            self.increase_counter_of(to)?;
            if self.token_owner.insert(id, *to).is_some() {
                return Err(Error::CannotInsert)
            }
            Ok(())
        }

        /// Approves or disapproves the operator to transfer all tokens of the caller.
        fn approve_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if to == caller {
                return Err(Error::NotAllowed)
            }
            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator: to,
                approved,
            });
            if self.approved_for_all(caller, to) {
                let status = self
                    .operator_approvals
                    .get_mut(&(caller, to))
                    .ok_or(Error::CannotFetchValue)?;
                *status = approved;
                Ok(())
            } else {
                match self.operator_approvals.insert((caller, to), approved) {
                    Some(_) => Err(Error::CannotInsert),
                    None => Ok(()),
                }
            }
        }

        /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
        fn approve_for(&mut self, to: &AccountId, id: u32) -> Result<(), Error> {         // u32 - TokenId
            let caller = self.env().caller();
            let owner = self.owner_of(id);
            if !(owner == Some(caller)
                || self.approved_for_all(owner.expect("Error with AccountId"), caller))
            {
                return Err(Error::NotAllowed)
            };
            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };

            if self.token_approvals.insert(id, *to).is_some() {
                return Err(Error::CannotInsert)
            };
            self.env().emit_event(Approval {
                from: caller,
                to: *to,
                id,
            });
            Ok(())
        }

        /// Increase token counter from the `of` AccountId.
        fn increase_counter_of(&mut self, of: &AccountId) -> Result<(), Error> {
            if self.balance_of_or_zero(of) > 0 {
                let count = self
                    .owned_tokens_count
                    .get_mut(of)
                    .ok_or(Error::CannotFetchValue)?;
                *count += 1;
                Ok(())
            } else {
                match self.owned_tokens_count.insert(*of, 1) {
                    Some(_) => Err(Error::CannotInsert),
                    None => Ok(()),
                }
            }
        }

        /// Decrease token counter from the `of` AccountId.
        fn decrease_counter_of(&mut self, of: &AccountId) -> Result<(), Error> {
            let count = self
                .owned_tokens_count
                .get_mut(of)
                .ok_or(Error::CannotFetchValue)?;
            *count -= 1;
            Ok(())
        }

        /// Removes existing approval from token `id`.
        fn clear_approval(&mut self, id: u32) -> Result<(), Error> {         // u32 - TokenId
            if !self.token_approvals.contains_key(&id) {
                return Ok(())
            };
            match self.token_approvals.remove(&id) {
                Some(_res) => Ok(()),
                None => Err(Error::CannotRemove),
            }
        }

        // Returns the total number of tokens from an account.
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            *self.owned_tokens_count.get(of).unwrap_or(&0)
        }

        /// Gets an operator on other Account's behalf.
        fn approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            *self
                .operator_approvals
                .get(&(owner, operator))
                .unwrap_or(&false)
        }

        /// Returns true if the AccountId `from` is the owner of token `id`
        /// or it has been approved on behalf of the token `id` owner.
        fn approved_or_owner(&self, from: Option<AccountId>, id: u32) -> bool {         // u32 - TokenId
            let owner = self.owner_of(id);
            from != Some(AccountId::from([0x0; 32]))
                && (from == owner
                    || from == self.token_approvals.get(&id).cloned()
                    || self.approved_for_all(
                        owner.expect("Error with AccountId"),
                        from.expect("Error with AccountId"),
                    ))
        }

        /// Returns true if token `id` exists or false if it does not.
        fn exists(&self, id: u32) -> bool {        // u32 - TokenId
            self.token_owner.get(&id).is_some() && self.token_owner.contains_key(&id)
        }

        /// Get account metadata or return empty if not initialized
        fn account_metadata_of_or_empty(&self, owner: &AccountId) -> Vec<u8> {
            let empty: Vec<u8> = Vec::new();
            let vec_value = self.account_metadata.get(owner).unwrap_or(&empty);
            vec_value.to_vec()
        }

        /// Get supply chain data or return empty if not initialized
        fn supply_chain_data_of_or_empty(&self, owner: &AccountId) -> Vec<u8> {
            let empty: Vec<u8> = Vec::new();
            let vec_value = self.supply_chain_data.get(owner).unwrap_or(&empty);
            vec_value.to_vec()
        }

        /// Get roles data or return empty if not initialized
        fn roles_data_of_or_empty(&self, owner: &AccountId) -> Vec<u8> {
            let empty: Vec<u8> = Vec::new();
            let vec_value = self.roles_data.get(owner).unwrap_or(&empty);
            vec_value.to_vec()
        }

        /// Get token meta data or return empty if not initialized
        fn token_metadata_of_or_empty(&self, token: &u32) -> Vec<u8> {
          let empty: Vec<u8> = Vec::new();
          let vec_value = self.token_metadata.get(token).unwrap_or(&empty);
          vec_value.to_vec()
      }
    }

    /// ========================================================================================================================
    /// ========================================================================================================================
    /// ========================================================================================================================
    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_core::env;

        #[test]
        fn new_works() {
          let contract = Erc721::new();
          assert_eq!(contract.version, 3);
        }

        #[test]
        fn mint_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Token 1 does not exists.
            assert_eq!(erc721.owner_of(1), None);
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Total tokens works = 0
            assert_eq!(erc721.list_all_tokens().len(), 0);
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Total tokens works = 1
            assert_eq!(erc721.list_all_tokens().len(), 1);
        }

        #[test]
        fn mint_existing_should_fail() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // The first Transfer event takes place
            assert_eq!(1, env::test::recorded_events().count());
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Cannot create  token Id if it exists.
            // Bob cannot own token Id 1.
            assert_eq!(erc721.mint(1), Err(Error::TokenExists));
        }

        #[test]
        fn transfer_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns token 1
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns any token
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // The first Transfer event takes place
            assert_eq!(1, env::test::recorded_events().count());
            // Alice transfers token 1 to Bob
            assert_eq!(erc721.transfer(accounts.bob, 1), Ok(()));
            // The second Transfer event takes place
            assert_eq!(2, env::test::recorded_events().count());
            // Bob owns token 1
            assert_eq!(erc721.balance_of(accounts.bob), 1);
        }

        #[test]
        fn invalid_transfer_should_fail() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Transfer token fails if it does not exists.
            assert_eq!(erc721.transfer(accounts.bob, 2), Err(Error::TokenNotFound));
            // Token Id 2 does not exists.
            assert_eq!(erc721.owner_of(2), None);
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Token Id 2 is owned by Alice.
            assert_eq!(erc721.owner_of(2), Some(accounts.alice));
            // Get contract address
            let callee =
                env::account_id::<env::DefaultEnvTypes>().unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                env::call::CallData::new(env::call::Selector::from_str("balance_of"));
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            assert_eq!(
                env::test::push_execution_context::<env::DefaultEnvTypes>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );
            // Bob cannot transfer not owned tokens.
            assert_eq!(erc721.transfer(accounts.eve, 2), Err(Error::NotApproved));
        }

        #[test]
        fn approved_transfer_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Token Id 1 is owned by Alice.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.approve(accounts.bob, 1), Ok(()));
            // Get contract address.
            let callee =
                env::account_id::<env::DefaultEnvTypes>().unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                env::call::CallData::new(env::call::Selector::from_str("balance_of"));
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            assert_eq!(
                env::test::push_execution_context::<env::DefaultEnvTypes>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 1),
                Ok(())
            );
            // TokenId 3 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 1 token.
            assert_eq!(erc721.balance_of(accounts.eve), 1);
        }

        #[test]
        fn approved_for_all_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 2);
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.set_approval_for_all(accounts.bob, true), Ok(()));
            // Bob is an approved operator for Alice
            assert_eq!(
                erc721.is_approved_for_all(accounts.alice, accounts.bob),
                true
            );
            // Get contract address.
            let callee =
                env::account_id::<env::DefaultEnvTypes>().unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                env::call::CallData::new(env::call::Selector::from_str("balance_of"));
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            assert_eq!(
                env::test::push_execution_context::<env::DefaultEnvTypes>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 1),
                Ok(())
            );
            // TokenId 1 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob transfers token Id 2 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 2),
                Ok(())
            );
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 2);
            // Get back to the parent execution context.
            env::test::pop_execution_context();
            // Remove operator approval for Bob on behalf of Alice.
            assert_eq!(erc721.set_approval_for_all(accounts.bob, false), Ok(()));
            // Bob is not an approved operator for Alice.
            assert_eq!(
                erc721.is_approved_for_all(accounts.alice, accounts.bob),
                false
            );
        }

        #[test]
        fn not_approved_transfer_should_fail() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
            // Get contract address.
            let callee =
                env::account_id::<env::DefaultEnvTypes>().unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                env::call::CallData::new(env::call::Selector::from_str("balance_of"));
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Eve as caller
            assert_eq!(
                env::test::push_execution_context::<env::DefaultEnvTypes>(
                    accounts.eve,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );
            // Eve is not an approved operator by Alice.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.frank, 1),
                Err(Error::NotApproved)
            );
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
        }

        #[test]
        fn burn_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Destroy token Id 1.
            assert_eq!(erc721.burn(1), Ok(()));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Token Id 1 does not exists
            assert_eq!(erc721.owner_of(1), None);
        }

        #[test]
        fn account_metadata_works() {
          let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
          .expect("Cannot get accounts");

          let mut contract = Erc721::new();

          // Initial value should be empty
          assert_eq!(contract.account_metadata_of_as_string(accounts.alice), String::from(""));

          // Set own value should work
          contract.set_account_metadata_of(accounts.alice, String::from("Alice metadata").into());
          assert_eq!(contract.account_metadata_of_as_string(accounts.alice), String::from("Alice metadata"));
        }

        #[test]
        fn supply_chain_works() {
          let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
          .expect("Cannot get accounts");

          let mut contract = Erc721::new();

          // Initial value should be empty
          assert_eq!(contract.supply_chain_as_string(), String::from(""));

          // Set and value should work
          contract.set_supply_chain(String::from("[\"Step1\", \"Step2\"]").into());
          assert_eq!(contract.supply_chain_as_string(), String::from("[\"Step1\", \"Step2\"]"));
        }

        #[test]
        fn roles_works() {
          let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
          .expect("Cannot get accounts");

          let mut contract = Erc721::new();

          // Initial value should be empty
          assert_eq!(contract.roles_as_string(), String::from(""));

          // Set and value should work
          contract.set_roles(String::from("[\"Role1\", \"Role2\"]").into());
          assert_eq!(contract.roles_as_string(), String::from("[\"Role1\", \"Role2\"]"));
        }

        #[test]
        fn token_metadata_works() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
            .expect("Cannot get accounts");

            let mut contract = Erc721::new();

            // Token 1 does not exists.
            assert_eq!(contract.owner_of(1), None);
            // Alice does not owns tokens.
            assert_eq!(contract.balance_of(accounts.alice), 0);
            // Total tokens works = 0
            assert_eq!(contract.list_all_tokens().len(), 0);
            // Create token Id 1.
            assert_eq!(contract.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(contract.balance_of(accounts.alice), 1);

            // Initial value should be empty
            assert_eq!(contract.token_metadata_of_as_string(1), String::from(""));

            // Set value should work
            contract.set_token_metadata_of(1, String::from("Token metadata").into());
            assert_eq!(contract.token_metadata_of_as_string(1), String::from("Token metadata"));
        }
    }
}
