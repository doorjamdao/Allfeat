use crate::{
	types::{CollectionConfig, CollectionSettings, MintSettings},
	AddressToCollectionId, NftsPrecompileSet, NftsPrecompileSetCall,
};
use core::marker::PhantomData;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, Everything},
	weights::Weight,
};
use pallet_evm::{EnsureAddressNever, EnsureAddressRoot, IdentityAddressMapping};
use pallet_nfts::PalletFeatures;
use precompile_utils::{
	mock_account,
	precompile_set::{AddressU64, PrecompileAt, PrecompileSetBuilder},
	testing::{AddressInPrefixedSet, MockAccount},
};
use sp_core::{ConstU128, ConstU32, ConstU64, H160, H256, U256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, Verify},
	BuildStorage,
};

pub(super) type AccountId = allfeat_primitives::AccountId;
type Signature = allfeat_primitives::Signature;
type AccountPublic = <Signature as Verify>::Signer;

pub type CollectionId = u128;
pub type Balance = u128;
pub type Block = frame_system::mocking::MockBlock<Runtime>;

/// The local Nfts precompile address prefix. Addresses that match against this prefix will
/// be routed to NftsPrecompileSet being marked as local
pub const NFTS_PRECOMPILE_ADDRESS_PREFIX: u32 = 0xfffffffe;

mock_account!(LocalCollectionId(CollectionId), |value: LocalCollectionId| {
	AddressInPrefixedSet(NFTS_PRECOMPILE_ADDRESS_PREFIX, value.0).into()
});

// Implement the trait, where we convert AccountId to CollectionId
impl AddressToCollectionId<CollectionId> for Runtime {
	/// The way to convert an account to collectionId is by ensuring that the prefix is 0XFFFFFFFF
	/// and by taking the lowest 128 bits as the collectionId
	fn address_to_collection_id(address: H160) -> Option<CollectionId> {
		let address: MockAccount = address.into();
		if address.has_prefix_u32(NFTS_PRECOMPILE_ADDRESS_PREFIX) {
			return Some(address.without_prefix());
		} else {
			None
		}
	}

	fn collection_id_to_address(collection_id: CollectionId) -> H160 {
		LocalCollectionId(collection_id).into()
	}
}

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(
			frame_support::weights::Weight::from_parts(1024, u64::MAX),
		);
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 0;
}

impl pallet_balances::Config for Runtime {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type MaxHolds = ();
}

const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

pub type PCall = NftsPrecompileSetCall<Runtime>;

parameter_types! {
	pub BlockGasLimit: U256 = U256::from(u64::MAX);
	pub GasLimitPovSizeRatio: u64 = {
		let block_gas_limit = BlockGasLimit::get().min(u64::MAX.into()).low_u64();
		block_gas_limit.saturating_div(MAX_POV_SIZE)
	};
	pub WeightPerGas: Weight = Weight::from_parts(1, 0);
	pub PrecompilesValue: NftsPrecompileSet<Runtime> =
		NftsPrecompileSet(PhantomData);
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
	type AddressMapping = IdentityAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = NftsPrecompileSet<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ();
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = ();
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type Timestamp = Timestamp;
	type SuicideQuickClearLimit = ();
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub storage Features: PalletFeatures = PalletFeatures::all_enabled();
}

impl pallet_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = CollectionId;
	type ItemId = u128;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Locker = ();
	type CollectionDeposit = ConstU128<2>;
	type ItemDeposit = ConstU128<1>;
	type MetadataDepositBase = ConstU128<1>;
	type AttributeDepositBase = ConstU128<1>;
	type DepositPerByte = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type ApprovalsLimit = ConstU32<10>;
	type ItemAttributesApprovalsLimit = ConstU32<2>;
	type MaxTips = ConstU32<10>;
	type MaxDeadlineDuration = ConstU64<10000>;
	type MaxAttributesPerCall = ConstU32<2>;
	type Features = Features;
	/// Off-chain = signature On-chain - therefore no conversion needed.
	/// It needs to be From<MultiSignature> for benchmarking.
	type OffchainSignature = Signature;
	/// Using `AccountPublic` here makes it trivial to convert to `AccountId` via `into_account()`.
	type OffchainPublic = AccountPublic;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

construct_runtime!(
	pub enum Runtime	{
		System: frame_system,
		Balances: pallet_balances,
		Evm: pallet_evm,
		Timestamp: pallet_timestamp,
		Nfts: pallet_nfts,
	}
);

pub(crate) struct ExtBuilder {
	// endowed accounts with balances
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> ExtBuilder {
		ExtBuilder { balances: vec![] }
	}
}

impl ExtBuilder {
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.expect("Frame system builds valid default genesis config");

		pallet_balances::GenesisConfig::<Runtime> { balances: self.balances }
			.assimilate_storage(&mut t)
			.expect("Pallet balances storage can be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub const ALICE: H160 = H160::repeat_byte(0xAA);

pub fn mock_collection_config() -> CollectionConfig {
	CollectionConfig {
		settings: CollectionSettings::all_enabled(),
		max_supply: Default::default(),
		mint_settings: MintSettings::item_settings_all_enabled(),
	}
}
