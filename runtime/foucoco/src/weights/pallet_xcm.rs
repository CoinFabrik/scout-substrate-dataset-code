
//! Autogenerated weights for `pallet_xcm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Bs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("foucoco"), DB CACHE: 1024

// Executed Command:
// ./target/production/pendulum-node
// benchmark
// pallet
// --chain
// foucoco
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-xcm
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallet_xcm.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn send() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `245`
		//  Estimated: `10630`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(27_000_000, 0)
			.saturating_add(Weight::from_parts(0, 10630))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn teleport_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn reserve_transfer_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:0 w:1)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn force_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm SafeXcmVersion (r:0 w:1)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	fn force_default_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: PolkadotXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_subscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `245`
		//  Estimated: `16315`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 0)
			.saturating_add(Weight::from_parts(0, 16315))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_unsubscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `15866`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(32_000_000, 0)
			.saturating_add(Weight::from_parts(0, 15866))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:4 w:2)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn migrate_supported_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229`
		//  Estimated: `11119`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11119))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notifiers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `11123`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11123))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:5 w:0)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn already_notified_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `13605`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 13605))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:2 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn notify_current_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `17217`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 0)
			.saturating_add(Weight::from_parts(0, 17217))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:3 w:0)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn notify_target_migration_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `8687`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8687))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notify_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `11130`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11130))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn migrate_and_notify_old_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `22191`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(38_000_000, 0)
			.saturating_add(Weight::from_parts(0, 22191))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}