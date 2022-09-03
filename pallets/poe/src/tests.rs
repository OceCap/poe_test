/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-09-03 21:26:59
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-09-03 23:50:39
 * @FilePath: \substrate-node-template\pallets\poe\src\tests.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_system::Origin;

#[test]
//CreateClaim
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim.clone()).unwrap();
		assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((1, frame_system::Pallet::<Test>::block_number())))
	});
}

#[test]
//ProofsAlreadyExist
fn create_claim_failed_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(2), claim.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
//RevokeClaim
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()), Ok(()));
	});
}

#[test]
//NotProofOwner
fn revoke_claim_failed_works1() {
	new_test_ext().execute_with(||{
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), 
			Error::<Test>::NotProofOwner
		);
	})
}

#[test]
//ProofNotExist
fn revoke_claim_failed_works2() {
	new_test_ext().execute_with(||{
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), 
			Error::<Test>::NoSuchProof
		);
	})
}

#[test]
//TransferSucceed
fn transer_claim_works() {
	new_test_ext().execute_with(||{
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		let dest = 1231231231 as u64;
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(PoeModule::transfer_claim(Origin::signed(1), dest, claim.clone()), Ok(()));
	})
}

#[test]
//Transferfailed_ProofNotExist
fn transer_claim_failed_works1() {
	new_test_ext().execute_with(||{
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		let dest = 1231231231 as u64;
		assert_eq!(PoeModule::transfer_claim(Origin::signed(1), dest, claim.clone()),
			Error::<Test>::NoSuchProof
		);
	})
}

#[test]
//Transferfailed_ProofNotExist
fn transer_claim_failed_works2() {
	new_test_ext().execute_with(||{
		let claim: BoundedVec<u8, <Test as Config>::MaxBytesInHash> = vec![0,1].try_into().unwrap();
		let dest = 1231231231 as u64;
		let sender: OriginFor<Test> = 1231231231;
		assert_eq!(PoeModule::transfer_claim(sender, dest, claim.clone()),
			Error::<Test>::TransferToSelf
		);
	})
}