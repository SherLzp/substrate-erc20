#![cfg_attr(not(feature = "std"), no_std)]
use rstd::prelude::*;
use support::{StorageValue, StorageMap, Parameter, decl_module, decl_event, decl_storage, ensure};
use runtime_primitives::traits::{Member, SimpleArithmetic, Zero, StaticLookup};
use system::{self, ensure_signed};

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The units in which we record balances.
    type Balance: Member + Parameter + SimpleArithmetic + Default + Copy;
}


type AssetId = u32;

decl_module! {
	// Simple declaration of the `Module` type. Lets the macro know what its working on.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;
		/// Issue a new class of fungible assets. There are, and will only ever be, `total`
		/// such assets and they'll all belong to the `origin` initially. It will have an
		/// identifier `AssetId` instance: this will be specified in the `Issued` event.
		fn issue(origin, #[compact] total: T::Balance) {
			let origin = ensure_signed(origin)?;

			let id = Self::next_asset_id();
			<NextAssetId<T>>::mutate(|id| *id += 1);

			<Balances<T>>::insert((id, origin.clone()), total);
			<TotalSupply<T>>::insert(id, total);

			Self::deposit_event(RawEvent::Issued(id, origin, total));
		}

		/// Move some assets from one holder to another.
		fn transfer(origin,
			#[compact] id: AssetId,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: T::Balance
		) {
			let origin = ensure_signed(origin)?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), "transfer amount should be non-zero");
			ensure!(origin_balance >= amount, "origin account balance must be greater than or equal to the transfer amount");

			Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate((id, target), |balance| *balance += amount);
		}

		/// Destroy any assets of `id` owned by `origin`.
		fn destroy(origin, #[compact] id: AssetId) {
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, origin.clone()));
			ensure!(!balance.is_zero(), "origin balance should be non-zero");

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId, <T as Trait>::Balance {
		/// Some assets were issued.
		Issued(AssetId, AccountId, Balance),
		/// Some assets were transferred.
		Transferred(AssetId, AccountId, AccountId, Balance),
		/// Some assets were destroyed.
		Destroyed(AssetId, AccountId, Balance),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Token {
		/// The number of units of assets held by any given account.
		Balances: map (AssetId, T::AccountId) => T::Balance;
		/// The next asset identifier up for grabs.
		NextAssetId get(next_asset_id): AssetId;
		/// The total unit supply of an asset
		TotalSupply: map AssetId => T::Balance;
	}
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
    // Public immutables

    /// Get the asset `id` balance of `who`.
    pub fn balance(id: AssetId, who: T::AccountId) -> T::Balance {
        <Balances<T>>::get((id, who))
    }

    // Get the total supply of an asset `id`
    pub fn total_supply(id: AssetId) -> T::Balance {
        <TotalSupply<T>>::get(id)
    }
}