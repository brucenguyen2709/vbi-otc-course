//! Benchmarking setup for pallet-assignment

use super::*;

#[allow(unused)]
use crate::Pallet as assignment;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_insert_cars {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Car::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(assignment, crate::mock::new_test_ext(), crate::mock::Test);
}
