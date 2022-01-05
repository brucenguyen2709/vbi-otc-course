#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Zero;
	use frame_support::sp_runtime::traits::SaturatedConversion;
	use frame_support::traits::Currency;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type LocalCurrency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn vehicle)]
	pub type Vehicle<T: Config> = StorageValue<_, u32>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VehicleStored(u32, T::AccountId),
		TotalInssuance(u32),
	}

	// Errors inform users that Vehicle went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		ExistVehicle,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn insert_car(origin: OriginFor<T>, code_of_vehicle: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let total_balance = T::LocalCurrency::total_issuance();
			let value = total_balance.saturated_into::<u32>();

			Self::deposit_event(Event::TotalInssuance(value));


			match <Vehicle<T>>::get() {
				None => {
					<Vehicle<T>>::put(code_of_vehicle);
					Self::deposit_event(Event::VehicleStored(code_of_vehicle, who));
				},
				Some(old) => {
					if old == code_of_vehicle {
						Err(Error::<T>::ExistVehicle)?;
					} else {
						<Vehicle<T>>::put(code_of_vehicle);
						Self::deposit_event(Event::VehicleStored(code_of_vehicle, who));
					}

				},
			}
			Ok(())
		}
	}
}
