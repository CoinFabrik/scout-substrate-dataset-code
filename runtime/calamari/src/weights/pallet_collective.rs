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

//! Autogenerated weights for pallet_collective
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_collective.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
    fn execute(b: u32, m: u32, ) -> Weight;
    fn propose_execute(b: u32, m: u32, ) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
    fn vote(m: u32, ) -> Weight;
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn close_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
    // Storage: Council Members (r:1 w:1)
    // Storage: Council Proposals (r:1 w:0)
    // Storage: Council Voting (r:100 w:100)
    // Storage: Council Prime (r:0 w:1)
    fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 9_000
            .saturating_add((10_408_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 9_000
            .saturating_add((13_811_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
    // Storage: Council Members (r:1 w:0)
    fn execute(b: u32, m: u32, ) -> Weight {
        (18_768_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 1_000
            .saturating_add((23_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32, ) -> Weight {
        (21_725_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 1_000
            .saturating_add((35_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalCount (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
        (21_190_000 as Weight)
            // Standard Error: 0
            .saturating_add((10_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((38_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((207_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Voting (r:1 w:1)
    fn vote(m: u32, ) -> Weight {
        (34_223_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((60_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
        (35_295_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((15_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((139_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
        (39_851_000 as Weight)
            // Standard Error: 0
            .saturating_add((5_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((42_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((182_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_disapproved(m: u32, p: u32, ) -> Weight {
        (36_649_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((38_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((134_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
        (38_815_000 as Weight)
            // Standard Error: 0
            .saturating_add((6_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((64_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((197_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32, ) -> Weight {
        (20_447_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((192_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Council Members (r:1 w:1)
    // Storage: Council Proposals (r:1 w:0)
    // Storage: Council Voting (r:100 w:100)
    // Storage: Council Prime (r:0 w:1)
    fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 9_000
            .saturating_add((10_408_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 9_000
            .saturating_add((13_811_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
    // Storage: Council Members (r:1 w:0)
    fn execute(b: u32, m: u32, ) -> Weight {
        (18_768_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 1_000
            .saturating_add((23_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32, ) -> Weight {
        (21_725_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 1_000
            .saturating_add((35_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalCount (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
        (21_190_000 as Weight)
            // Standard Error: 0
            .saturating_add((10_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((38_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((207_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Voting (r:1 w:1)
    fn vote(m: u32, ) -> Weight {
        (34_223_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((60_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
        (35_295_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((15_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((139_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
        (39_851_000 as Weight)
            // Standard Error: 0
            .saturating_add((5_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((42_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((182_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_disapproved(m: u32, p: u32, ) -> Weight {
        (36_649_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((38_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((134_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
        (38_815_000 as Weight)
            // Standard Error: 0
            .saturating_add((6_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 2_000
            .saturating_add((64_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 2_000
            .saturating_add((197_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32, ) -> Weight {
        (20_447_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((192_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
}