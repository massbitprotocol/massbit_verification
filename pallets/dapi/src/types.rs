use super::*;
use frame_support::pallet_prelude::*;

#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Project<AccountId, ChainId> {
	pub owner: AccountId,
	pub chain_id: ChainId,
	pub quota: u128,
	pub usage: u128,
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum ProviderType {
	Gateway,
	Node,
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum ProviderState {
	Registered,
	Unregistered,
}

#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Provider<AccountId, ChainId> {
	pub provider_type: ProviderType,
	pub operator: AccountId,
	pub chain_id: ChainId,
	pub state: ProviderState,
}
