// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_operation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-22, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-operation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/operation/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_operation.
pub trait WeightInfo {
    fn force_reserve_by_member() -> Weight;
    fn force_remove_lock() -> Weight;
    fn set_reserve_members() -> Weight;
    fn set_release_owner_address() -> Weight;
    fn set_release_limit_parameter() -> Weight;
    fn unstaking_release() -> Weight;
    fn burn_for_ezc() -> Weight;
    fn npow_mint() -> Weight;
    fn bridge_deeper_to_other() -> Weight;
    fn bridge_other_to_deeper() -> Weight;
    fn set_dpr_price() -> Weight;
}

/// Weights for pallet_operation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn force_reserve_by_member() -> Weight {
        (29_576_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn force_remove_lock() -> Weight {
        (29_387_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn set_reserve_members() -> Weight {
        (2_525_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_release_owner_address() -> Weight {
        (1_573_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_release_limit_parameter() -> Weight {
        (2_154_000 as Weight).saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn unstaking_release() -> Weight {
        (16_301_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn burn_for_ezc() -> Weight {
        (41_810_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn npow_mint() -> Weight {
        (41_810_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn bridge_deeper_to_other() -> Weight {
        (38_885_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn bridge_other_to_deeper() -> Weight {
        (38_794_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn set_dpr_price() -> Weight {
        (31_350_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn force_reserve_by_member() -> Weight {
        (29_576_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn force_remove_lock() -> Weight {
        (29_387_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn set_reserve_members() -> Weight {
        (2_525_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_release_owner_address() -> Weight {
        (1_573_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_release_limit_parameter() -> Weight {
        (2_154_000 as Weight).saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn unstaking_release() -> Weight {
        (16_301_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }

    fn burn_for_ezc() -> Weight {
        (41_810_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn npow_mint() -> Weight {
        (41_810_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn bridge_deeper_to_other() -> Weight {
        (38_885_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn bridge_other_to_deeper() -> Weight {
        (38_794_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn set_dpr_price() -> Weight {
        (31_350_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
