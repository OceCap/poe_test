/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-06-06 22:09:37
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-09-03 22:48:40
 * @FilePath: \substrate-node-template\pallets\template\src\lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
#[frame_support::pallet]

pub mod pallet{
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// TODO: Update the `config` block below
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event : From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MaxBytesInHash: Get<u32>;
	}
	// TODO: Update the `event` block below
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		TransferSucceed(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}
	// TODO: Update the `error` block below
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
		TransferToSelf,
		TranserFailed,
	}

	// TODO: add #[pallet::storage] block
	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<_, 
	Blake2_128Concat,
	BoundedVec<u8, T::MaxBytesInHash>,
	(T::AccountId, T::BlockNumber),
	OptionQuery,
	>;
    // TODO: Update the `call` block below
	#[pallet::call]
	impl <T:Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();
			
			Proofs::<T>::insert(&proof, (&sender, current_block));
			Self::deposit_event(Event::ClaimCreated(sender, proof));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim (
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof),Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof)
				.expect("All proofs must have an owner!");

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);
			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(())
		}

		#[pallet::weight(100)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            to: T::AccountId,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            ensure!(sender != to, Error::<T>::TransferToSelf);
			let current_block = <frame_system::Pallet<T>>::block_number();         
			Proofs::<T>::remove(&proof);
			
			
			Proofs::<T>::insert(&proof, (&to.clone(), current_block)); 
            Self::deposit_event(Event::TransferSucceed(sender, to, proof));
            Ok(())
        }
	}
}
