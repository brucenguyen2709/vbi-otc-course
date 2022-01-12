#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	use scale_info::TypeInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct WatterContract {
		pub code: u32,
		pub proof: Vec<u8>,
	}

	#[pallet::storage]
	pub(super) type WatterContracts<T: Config> = StorageMap<_, Blake2_128Concat, WatterContract, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, WatterContract),
		ClaimRevoked(T::AccountId, WatterContract),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_contract(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			code: u32
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_contract = WatterContract {
				code,
				proof,
			};

			// println!("{:?}", &new_contract.code);
			// println!("{:?}", &new_contract.proof);

			ensure!(!WatterContracts::<T>::contains_key(&new_contract), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();

			WatterContracts::<T>::insert(&new_contract, (&sender, current_block));

			Self::deposit_event(Event::ClaimCreated(sender, new_contract));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_contract(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			code: u32
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let old_contract = WatterContract {
				code,
				proof,
			};

			ensure!(WatterContracts::<T>::contains_key(&old_contract), Error::<T>::NoSuchProof);

			let (owner, _) = WatterContracts::<T>::get(&old_contract);

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			WatterContracts::<T>::remove(&old_contract);

			Self::deposit_event(Event::ClaimRevoked(sender, old_contract));

			Ok(().into())
		}
	}
}
