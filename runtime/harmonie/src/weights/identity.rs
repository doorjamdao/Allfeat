
//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `harmonie-node-01`, CPU: `Intel(R) Xeon(R) Platinum 8280 CPU @ 2.70GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/harmonie/src/weights/identity.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn add_registrar(r: u32, ) -> Weight;
	fn set_identity(r: u32, ) -> Weight;
	fn set_subs_new(s: u32, ) -> Weight;
	fn set_subs_old(p: u32, ) -> Weight;
	fn clear_identity(r: u32, s: u32, ) -> Weight;
	fn request_judgement(r: u32, ) -> Weight;
	fn cancel_request(r: u32, ) -> Weight;
	fn set_fee(r: u32, ) -> Weight;
	fn set_account_id(r: u32, ) -> Weight;
	fn set_fields(r: u32, ) -> Weight;
	fn provide_judgement(r: u32, ) -> Weight;
	fn kill_identity(r: u32, s: u32, ) -> Weight;
	fn add_sub(s: u32, ) -> Weight;
	fn rename_sub(s: u32, ) -> Weight;
	fn remove_sub(s: u32, ) -> Weight;
	fn quit_sub(s: u32, ) -> Weight;
}

/// Weights for pallet_identity using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for AllfeatWeight<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 14_816_000 picoseconds.
		Weight::from_parts(15_514_865, 2386)
			// Standard Error: 3_166
			.saturating_add(Weight::from_parts(140_674, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7076 + r * (5 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 173_689_000 picoseconds.
		Weight::from_parts(192_096_973, 11106)
			// Standard Error: 28_646
			.saturating_add(Weight::from_parts(64_583, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89`
		//  Estimated: `11106 + s * (2565 ±0)`
		// Minimum execution time: 15_479_000 picoseconds.
		Weight::from_parts(38_760_730, 11106)
			// Standard Error: 9_989
			.saturating_add(Weight::from_parts(5_163_676, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2565).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169 + p * (20 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 14_627_000 picoseconds.
		Weight::from_parts(37_849_293, 11106)
			// Standard Error: 6_125
			.saturating_add(Weight::from_parts(2_071_681, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7156 + r * (5 ±0) + s * (20 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 88_030_000 picoseconds.
		Weight::from_parts(81_264_569, 11106)
			// Standard Error: 43_245
			.saturating_add(Weight::from_parts(563_322, 0).saturating_mul(r.into()))
			// Standard Error: 8_438
			.saturating_add(Weight::from_parts(2_071_921, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7066 + r * (45 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 118_745_000 picoseconds.
		Weight::from_parts(129_316_903, 11106)
			// Standard Error: 12_002
			.saturating_add(Weight::from_parts(94_010, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7097`
		//  Estimated: `11106`
		// Minimum execution time: 116_928_000 picoseconds.
		Weight::from_parts(127_720_643, 11106)
			// Standard Error: 20_881
			.saturating_add(Weight::from_parts(70_521, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 10_480_000 picoseconds.
		Weight::from_parts(11_107_521, 2386)
			// Standard Error: 1_723
			.saturating_add(Weight::from_parts(106_779, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 9_751_000 picoseconds.
		Weight::from_parts(10_237_147, 2386)
			// Standard Error: 1_796
			.saturating_add(Weight::from_parts(89_936, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 9_715_000 picoseconds.
		Weight::from_parts(10_192_644, 2386)
			// Standard Error: 1_611
			.saturating_add(Weight::from_parts(88_243, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7132 + r * (45 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 146_584_000 picoseconds.
		Weight::from_parts(160_363_676, 11106)
			// Standard Error: 20_685
			.saturating_add(Weight::from_parts(38_801, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7284 + r * (5 ±0) + s * (20 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 94_634_000 picoseconds.
		Weight::from_parts(89_226_924, 11106)
			// Standard Error: 44_991
			.saturating_add(Weight::from_parts(704_291, 0).saturating_mul(r.into()))
			// Standard Error: 8_778
			.saturating_add(Weight::from_parts(2_030_559, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `536 + s * (23 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 43_915_000 picoseconds.
		Weight::from_parts(49_627_951, 11106)
			// Standard Error: 2_361
			.saturating_add(Weight::from_parts(119_321, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `615 + s * (4 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 19_464_000 picoseconds.
		Weight::from_parts(22_890_607, 11106)
			// Standard Error: 1_371
			.saturating_add(Weight::from_parts(70_114, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7641), added: 10116, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647 + s * (24 ±0)`
		//  Estimated: `11106`
		// Minimum execution time: 47_559_000 picoseconds.
		Weight::from_parts(52_303_199, 11106)
			// Standard Error: 3_653
			.saturating_add(Weight::from_parts(115_304, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622 + s * (24 ±0)`
		//  Estimated: `5511`
		// Minimum execution time: 35_347_000 picoseconds.
		Weight::from_parts(37_731_912, 5511)
			// Standard Error: 1_729
			.saturating_add(Weight::from_parts(105_743, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
