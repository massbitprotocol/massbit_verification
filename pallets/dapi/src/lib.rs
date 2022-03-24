#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;
use scale_info::TypeInfo;
use sp_runtime::traits::IsMember;
use sp_std::prelude::*;

use pallet_dapi_staking::Staking;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	type ProviderId = [u8; 36];
	type BlockChain = Vec<u8>;

	#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
	pub struct Project<AccountId> {
		pub owner: AccountId,
		pub blockchain: BlockChain,
		pub quota: u128,
		pub usage: u128,
	}

	#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum ProviderType {
		Gateway,
		Node,
	}

	#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
	pub struct Provider<AccountId> {
		pub provider_type: ProviderType,
		pub operator: AccountId,
		pub blockchain: BlockChain,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		type Staking: Staking<Self::AccountId, ProviderId>;

		type IsOracle: IsMember<Self::AccountId>;

		type IsFisherman: IsMember<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::error]
	pub enum Error<T> {
		AlreadyRegistered,
		InsufficientBoding,
		ProjectNotFound,
		NotOracle,
		NotFisherman,
		ProviderNotExist,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A project is successfully registered. \[project_id, account_id, blockchain, quota\]
		ProjectRegistered(ProviderId, T::AccountId, BlockChain, u128),
		/// A provider is successfully registered. \[provider_id, provider_type, operator,
		/// blockchain\]
		ProviderRegistered(ProviderId, ProviderType, T::AccountId, BlockChain),
		/// Project usage is reported.
		ProjectUsageReported(ProviderId, u128),
		/// Provide performance is reported.
		ProviderPerformanceReported(ProviderId, ProviderType, u64, u32, u32),
	}

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub(super) type Projects<T: Config> =
		StorageMap<_, Blake2_128Concat, ProviderId, Project<AccountIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn providers)]
	pub(super) type Providers<T: Config> =
		StorageMap<_, Blake2_128Concat, ProviderId, Provider<AccountIdOf<T>>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn register_project(
			origin: OriginFor<T>,
			project_id: ProviderId,
			blockchain: BlockChain,
			deposit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			ensure!(<Projects<T>>::get(&project_id).is_none(), Error::<T>::AlreadyRegistered);

			let quota = Self::calculate_consumer_quota(deposit);
			<Projects<T>>::insert(
				&project_id,
				Project { owner: account.clone(), blockchain: blockchain.clone(), quota, usage: 0 },
			);

			Self::deposit_event(Event::ProjectRegistered(project_id, account, blockchain, quota));

			Ok(().into())
		}

		#[pallet::weight(100)]
		pub fn register_provider(
			origin: OriginFor<T>,
			provider_id: ProviderId,
			provider_type: ProviderType,
			blockchain: BlockChain,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			ensure!(<Providers<T>>::get(&provider_id).is_none(), Error::<T>::AlreadyRegistered);

			T::Staking::register(account.clone(), provider_id.clone())?;

			<Providers<T>>::insert(
				&provider_id,
				Provider {
					provider_type,
					operator: account.clone(),
					blockchain: blockchain.clone(),
				},
			);

			Self::deposit_event(Event::ProviderRegistered(
				provider_id,
				provider_type,
				account,
				blockchain,
			));

			Ok(().into())
		}

		#[pallet::weight(100)]
		pub fn submit_project_usage(
			origin: OriginFor<T>,
			project_id: ProviderId,
			usage: u128,
		) -> DispatchResultWithPostInfo {
			let oracle = ensure_signed(origin)?;

			ensure!(T::IsOracle::is_member(&oracle), Error::<T>::NotOracle);

			Projects::<T>::mutate(&project_id, |project| {
				if let Some(project) = project {
					project.usage = project.usage.saturating_add(usage)
				}
			});

			Self::deposit_event(Event::ProjectUsageReported(project_id, usage));

			Ok(().into())
		}

		#[pallet::weight(100)]
		pub fn submit_provider_report(
			origin: OriginFor<T>,
			provider_id: ProviderId,
			requests: u64,
			success_percentage: u32,
			average_latency: u32,
		) -> DispatchResultWithPostInfo {
			let fishermen = ensure_signed(origin)?;

			ensure!(T::IsFisherman::is_member(&fishermen), Error::<T>::NotFisherman);

			let provider = Self::providers(&provider_id).ok_or(Error::<T>::ProviderNotExist)?;

			Self::deposit_event(Event::ProviderPerformanceReported(
				provider_id,
				provider.provider_type,
				requests,
				success_percentage,
				average_latency,
			));

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn calculate_consumer_quota(amount: BalanceOf<T>) -> u128 {
			TryInto::<u128>::try_into(amount).ok().unwrap_or_default()
		}
	}
}
