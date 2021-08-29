use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_defaulet_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(Origin::signed(1)));
        assert_noop!(Kitties::transfer(Origin::signed(1), 1, 2), Error::<Test>::NotOwner);
    });
}