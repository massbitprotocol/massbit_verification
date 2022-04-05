//! Autogenerated weights for pallet_dapi_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-05, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/massbit-collator
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_dapi_staking
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./pallets/dapi-staking/src/weights.rs
// --template
// ./benchmarking/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_dapi_staking.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn withdraw_from_unregistered_staker() -> Weight;
	#[rustfmt::skip]
	fn withdraw_from_unregistered_operator() -> Weight;
	#[rustfmt::skip]
	fn stake() -> Weight;
	#[rustfmt::skip]
	fn unstake() -> Weight;
	#[rustfmt::skip]
	fn withdraw_unstaked() -> Weight;
	#[rustfmt::skip]
	fn claim_staker() -> Weight;
	#[rustfmt::skip]
	fn claim_operator() -> Weight;
	#[rustfmt::skip]
	fn force_new_era() -> Weight;
}

/// Weights for pallet_dapi_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_from_unregistered_staker() -> Weight {
		(66_746_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_from_unregistered_operator() -> Weight {
		(42_209_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	#[rustfmt::skip]
	fn stake() -> Weight {
		(74_039_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn unstake() -> Weight {
		(63_179_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_unstaked() -> Weight {
		(47_399_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:0)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:0)
	#[rustfmt::skip]
	fn claim_staker() -> Weight {
		(27_713_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:0)
	#[rustfmt::skip]
	fn claim_operator() -> Weight {
		(23_564_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DapiStaking ForceEra (r:0 w:1)
	#[rustfmt::skip]
	fn force_new_era() -> Weight {
		(1_132_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_from_unregistered_staker() -> Weight {
		(66_746_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_from_unregistered_operator() -> Weight {
		(42_209_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	#[rustfmt::skip]
	fn stake() -> Weight {
		(74_039_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn unstake() -> Weight {
		(63_179_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: DapiStaking Ledger (r:1 w:1)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:1)
	#[rustfmt::skip]
	fn withdraw_unstaked() -> Weight {
		(47_399_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: DapiStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:0)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:0)
	#[rustfmt::skip]
	fn claim_staker() -> Weight {
		(27_713_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: DapiStaking RegisteredProviders (r:1 w:0)
	// Storage: DapiStaking CurrentEra (r:1 w:0)
	// Storage: DapiStaking ProviderEraStake (r:1 w:1)
	// Storage: DapiStaking GeneralEraInfo (r:1 w:0)
	#[rustfmt::skip]
	fn claim_operator() -> Weight {
		(23_564_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: DapiStaking ForceEra (r:0 w:1)
	#[rustfmt::skip]
	fn force_new_era() -> Weight {
		(1_132_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
