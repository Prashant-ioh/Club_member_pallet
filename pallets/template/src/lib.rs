#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

<<<<<<< HEAD
=======
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
>>>>>>> 700c3a186e55d0d6542c564ccff17260238ed752
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubmembers)]
	pub type ClubMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn accuser)]
	pub type AccUser<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The member is add in the club.
		MemberAdded,
		/// The member is removed from the club.
		MemberRemoved,
		MemberRemovedSELF,

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already a member.
		AlreadyMember,
		/// Not a member.
		NotMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Add a member in the club.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

			club_members.insert(location, who.clone());

			ClubMembers::<T>::put(&club_members);


			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}

		/// Remove a member from the club.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;
			club_members.remove(location);

			ClubMembers::<T>::put(&club_members);


			Self::deposit_event(Event::MemberRemoved);
			Ok(())
		}
        
		
		// usser remove self
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_member_self(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = AccUser::<T>::get();
			let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;
			club_members.remove(location);

			AccUser::<T>::put(&club_members);

			
			Self::deposit_event(Event::MemberRemovedSELF);
			Ok(())
		}
		// Remove seft user
        // #[pallet::weight(10_00 + T::DbWeight::get().reads_writes(1,1))]

		// pub fn seft_remove(origin:OriginFor<T>, who:T:AccountId.get().Self) ->DispatchResult
		// {

		// }

		
	}
}
