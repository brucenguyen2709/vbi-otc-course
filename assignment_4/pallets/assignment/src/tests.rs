use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_contract_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(AssignmentModule::create_contract(Origin::signed(1), vec![1], 42));
	});
}
