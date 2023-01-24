// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_scheduler.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_scheduler.
pub trait WeightInfo {
    fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight;
    fn on_initialize_named_resolved(s: u32, ) -> Weight;
    fn on_initialize_periodic_resolved(s: u32, ) -> Weight;
    fn on_initialize_resolved(s: u32, ) -> Weight;
    fn on_initialize_named_aborted(s: u32, ) -> Weight;
    fn on_initialize_aborted(s: u32, ) -> Weight;
    fn on_initialize_periodic_named(s: u32, ) -> Weight;
    fn on_initialize_periodic(s: u32, ) -> Weight;
    fn on_initialize_named(s: u32, ) -> Weight;
    fn on_initialize(s: u32, ) -> Weight;
    fn schedule(s: u32, ) -> Weight;
    fn cancel(s: u32, ) -> Weight;
    fn schedule_named(s: u32, ) -> Weight;
    fn cancel_named(s: u32, ) -> Weight;
}

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for SubstrateWeight<T> {
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
        (16_078_000 as Weight)
            // Standard Error: 48_000
            .saturating_add((24_548_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_resolved(s: u32, ) -> Weight {
        (14_897_000 as Weight)
            // Standard Error: 38_000
            .saturating_add((19_354_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
        (12_459_000 as Weight)
            // Standard Error: 49_000
            .saturating_add((21_685_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_resolved(s: u32, ) -> Weight {
        (11_444_000 as Weight)
            // Standard Error: 47_000
            .saturating_add((18_361_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_aborted(s: u32, ) -> Weight {
        (5_725_000 as Weight)
            // Standard Error: 25_000
            .saturating_add((7_624_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    fn on_initialize_aborted(s: u32, ) -> Weight {
        (7_579_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((4_725_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named(s: u32, ) -> Weight {
        (15_946_000 as Weight)
            // Standard Error: 38_000
            .saturating_add((13_166_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    fn on_initialize_periodic(s: u32, ) -> Weight {
        (13_429_000 as Weight)
            // Standard Error: 33_000
            .saturating_add((10_306_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named(s: u32, ) -> Weight {
        (14_227_000 as Weight)
            // Standard Error: 25_000
            .saturating_add((8_176_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn on_initialize(s: u32, ) -> Weight {
        (13_882_000 as Weight)
            // Standard Error: 24_000
            .saturating_add((6_965_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule(s: u32, ) -> Weight {
        (19_202_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((101_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn cancel(s: u32, ) -> Weight {
        (19_210_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((1_041_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule_named(s: u32, ) -> Weight {
        (22_816_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((123_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_named(s: u32, ) -> Weight {
        (21_016_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((1_040_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
        (16_078_000 as Weight)
            // Standard Error: 48_000
            .saturating_add((24_548_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_resolved(s: u32, ) -> Weight {
        (14_897_000 as Weight)
            // Standard Error: 38_000
            .saturating_add((19_354_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
        (12_459_000 as Weight)
            // Standard Error: 49_000
            .saturating_add((21_685_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_resolved(s: u32, ) -> Weight {
        (11_444_000 as Weight)
            // Standard Error: 47_000
            .saturating_add((18_361_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_aborted(s: u32, ) -> Weight {
        (5_725_000 as Weight)
            // Standard Error: 25_000
            .saturating_add((7_624_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    fn on_initialize_aborted(s: u32, ) -> Weight {
        (7_579_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((4_725_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named(s: u32, ) -> Weight {
        (15_946_000 as Weight)
            // Standard Error: 38_000
            .saturating_add((13_166_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    fn on_initialize_periodic(s: u32, ) -> Weight {
        (13_429_000 as Weight)
            // Standard Error: 33_000
            .saturating_add((10_306_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named(s: u32, ) -> Weight {
        (14_227_000 as Weight)
            // Standard Error: 25_000
            .saturating_add((8_176_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn on_initialize(s: u32, ) -> Weight {
        (13_882_000 as Weight)
            // Standard Error: 24_000
            .saturating_add((6_965_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule(s: u32, ) -> Weight {
        (19_202_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((101_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn cancel(s: u32, ) -> Weight {
        (19_210_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((1_041_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule_named(s: u32, ) -> Weight {
        (22_816_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((123_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_named(s: u32, ) -> Weight {
        (21_016_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((1_040_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
}