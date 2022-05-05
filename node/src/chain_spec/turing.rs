use hex_literal::hex;

use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_core::{crypto::UncheckedInto, sr25519};

use super::TELEMETRY_URL;
use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, validate_allocation, validate_vesting,
	Extensions,
};
use primitives::{AccountId, AuraId, Balance};
use turing_runtime::{
	CouncilConfig, SudoConfig, TechnicalMembershipConfig, ValveConfig, VestingConfig, DOLLAR,
	EXISTENTIAL_DEPOSIT, TOKEN_DECIMALS,
};

static TOKEN_SYMBOL: &str = "TUR";
const SS_58_FORMAT: u32 = 51;
static RELAY_CHAIN: &str = "rococo-local";
static STAGING_RELAY_CHAIN: &str = "rococo-testnet";
static TURING_RELAY_CHAIN: &str = "ksmcc3";
const DEFAULT_PARA_ID: u32 = 2000;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<turing_runtime::GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> turing_runtime::SessionKeys {
	turing_runtime::SessionKeys { aura: keys }
}

pub fn turing_development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	ChainSpec::from_genesis(
		// Name
		"Turing Development",
		// ID
		"turing-dev",
		ChainType::Development,
		move || {
			let accounts = vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			];
			const ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
			let initial_balance: u128 = ALLOC_TOKENS_TOTAL / accounts.len() as u128;
			let endowed_accounts: Vec<(AccountId, Balance)> =
				accounts.iter().cloned().map(|k| (k, initial_balance)).collect();

			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				endowed_accounts,
				DEFAULT_PARA_ID.into(),
				vec![b"AutomationTime".to_vec(), b"Balances".to_vec(), b"Democracy".to_vec()],
				vec![],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: RELAY_CHAIN.into(), // You MUST set this to the correct network!
			para_id: DEFAULT_PARA_ID,
		},
	)
}

pub fn turing_staging() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	ChainSpec::from_genesis(
		// Name
		"Turing Network",
		// ID
		"turing",
		ChainType::Live,
		move || {
			let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
			let initial_allocation: Vec<(AccountId, Balance)> =
				serde_json::from_slice(allocation_json).unwrap();
			const ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
			validate_allocation(
				initial_allocation.clone(),
				ALLOC_TOKENS_TOTAL,
				EXISTENTIAL_DEPOSIT,
			);

			let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
			let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
				serde_json::from_slice(vesting_json).unwrap();

			let vested_tokens = 9_419_999_999_999_999_919;
			let vest_starting_time: u64 = 1651431600;
			let vest_ending_time: u64 = 1743534000;
			validate_vesting(
				initial_vesting.clone(),
				vested_tokens,
				EXISTENTIAL_DEPOSIT,
				vest_starting_time,
				vest_ending_time,
			);

			testnet_genesis(
				// initial collators.
				vec![
					(
						// SS58 prefix 51: 68eS5bmdz2nY45KxBfBFePVH88xupaPvjzExMxcnGuRBLcSK
						// SS58 prefix substrate: 5CdDSsMpHjCkdqX49iyaozNrnLEZ4xUkk2mX4N6cVbxxpYhy
						hex!["6e6dd706f9c325ea3d316392135766c0b16feb4fd6fa3171ae2da5f00f022a0a"]
							.into(),
						hex!["6e6dd706f9c325ea3d316392135766c0b16feb4fd6fa3171ae2da5f00f022a0a"]
							.unchecked_into(),
					),
					(
						// SS58 prefix 51: 6BKgyjBS7exBgGKCVMGbYYcdTLHnXdJFHXD86Vr3vX4vpzPz
						// SS58 prefix substrate: 5HWVKNhY1PcSWcpSX5QgXJ7iYxJkAubDxxBB4p2v4wBRXQkE
						hex!["e4d7080396ff3c0d501d18d33afee4e3140a6a878e6277968bf3771fe6cd4956"]
							.into(),
						hex!["e4d7080396ff3c0d501d18d33afee4e3140a6a878e6277968bf3771fe6cd4956"]
							.unchecked_into(),
					),
				],
				// 5CGLdvHNAMRZM3T52crEcP46YTsmq2GVke5wc6w3Z1epBDVy
				hex!["08df8338e854d8d589dedd4305c11e589cbef994e5dd00c7bb8fb7d277705b06"].into(),
				initial_allocation,
				DEFAULT_PARA_ID.into(),
				vec![b"AutomationTime".to_vec(), b"Balances".to_vec(), b"Democracy".to_vec()],
				initial_vesting,
				vec![
					// 67nmVh57G9yo7sqiGLjgNNqtUd7H2CSESTyQgp5272aMibwS
					hex!["488ced7d199b4386081a52505962128da5a3f54f4665db3d78b6e9f9e89eea4d"].into(),
					// 67kgfmY6zpw1PRYpj3D5RtkzVZnVvn49XHGyR4v9MEsRRyet
					hex!["46f630b3f79c588100dc0f69845633a830e01ea09eed4f1d01314a9bf33b9c16"].into(),
					// 67D6ecyNhnAzZqgRbxr3MdGnxB9Bw8VadMhjpLAYB3wf5Pq6
					hex!["2edf0fd8948ea642f135b314b1358c77ec6d0a4af83220b6ea18136e5ce36277"].into(),
					// 6AMsXyV1CYc3LMTk155JTDGEzbgVPvsX9aXp7VXz9heC3iuP
					hex!["ba44d2c00d9528c2d1fc51cef8ce8b9c3939928ecda8f404cdc46e3a2c090627"].into(),
				],
				vec![
					// 67nmVh57G9yo7sqiGLjgNNqtUd7H2CSESTyQgp5272aMibwS
					hex!["488ced7d199b4386081a52505962128da5a3f54f4665db3d78b6e9f9e89eea4d"].into(),
					// 67kgfmY6zpw1PRYpj3D5RtkzVZnVvn49XHGyR4v9MEsRRyet
					hex!["46f630b3f79c588100dc0f69845633a830e01ea09eed4f1d01314a9bf33b9c16"].into(),
					// 6A6VuGbeUwm3J2HqLduH7VFZTvrYQs8GuqzdhopLGN2JKMAe
					hex!["ae8b51cd0aa290645e593a4f54673ae62bab95791a137b943723bb6070533830"].into(),
					// 699YyPF2uA83zsFnQU4GCAvZXzucvyS5rx8LS9UrL9kEv8PP
					hex!["84a328f5f568d82ecd91861df7eae1065c1a2f1bcfec0950d4124e9363205b4a"].into(),
					// 669ocRxey7vxUJs1TTRWe31zwrpGr8B13zRfAHB6yhhfcMud
					hex!["001fbcefa8c96f3d2e236688da5485a0af67988b78d61ea952f461255d1f4267"].into(),
				],
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// Protocol ID
		Some("turing"),
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: STAGING_RELAY_CHAIN.into(), // You MUST set this to the correct network!
			para_id: DEFAULT_PARA_ID,
		},
	)
}

pub fn turing_live() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	let live_para_id = 2114;

	ChainSpec::from_genesis(
		// Name
		"Turing Network",
		// ID
		"turing",
		ChainType::Live,
		move || {
			let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
			let initial_allocation: Vec<(AccountId, Balance)> =
				serde_json::from_slice(allocation_json).unwrap();
			const ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
			validate_allocation(
				initial_allocation.clone(),
				ALLOC_TOKENS_TOTAL,
				EXISTENTIAL_DEPOSIT,
			);

			let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
			let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
				serde_json::from_slice(vesting_json).unwrap();

			let vested_tokens = 9_419_999_999_999_999_919;
			let vest_starting_time: u64 = 1651431600;
			let vest_ending_time: u64 = 1743534000;
			validate_vesting(
				initial_vesting.clone(),
				vested_tokens,
				EXISTENTIAL_DEPOSIT,
				vest_starting_time,
				vest_ending_time,
			);

			testnet_genesis(
				// initial collators.
				vec![
					(
						// SS58 prefix 51: 68eS5bmdz2nY45KxBfBFePVH88xupaPvjzExMxcnGuRBLcSK
						// SS58 prefix substrate: 5CdDSsMpHjCkdqX49iyaozNrnLEZ4xUkk2mX4N6cVbxxpYhy
						hex!["6e6dd706f9c325ea3d316392135766c0b16feb4fd6fa3171ae2da5f00f022a0a"]
							.into(),
						hex!["6e6dd706f9c325ea3d316392135766c0b16feb4fd6fa3171ae2da5f00f022a0a"]
							.unchecked_into(),
					),
					(
						// SS58 prefix 51: 6BKgyjBS7exBgGKCVMGbYYcdTLHnXdJFHXD86Vr3vX4vpzPz
						// SS58 prefix substrate: 5HWVKNhY1PcSWcpSX5QgXJ7iYxJkAubDxxBB4p2v4wBRXQkE
						hex!["e4d7080396ff3c0d501d18d33afee4e3140a6a878e6277968bf3771fe6cd4956"]
							.into(),
						hex!["e4d7080396ff3c0d501d18d33afee4e3140a6a878e6277968bf3771fe6cd4956"]
							.unchecked_into(),
					),
				],
				// 5CGLdvHNAMRZM3T52crEcP46YTsmq2GVke5wc6w3Z1epBDVy
				hex!["08df8338e854d8d589dedd4305c11e589cbef994e5dd00c7bb8fb7d277705b06"].into(),
				initial_allocation,
				live_para_id.into(),
				vec![b"AutomationTime".to_vec(), b"Balances".to_vec(), b"Democracy".to_vec()],
				initial_vesting,
				vec![
					// 67nmVh57G9yo7sqiGLjgNNqtUd7H2CSESTyQgp5272aMibwS
					hex!["488ced7d199b4386081a52505962128da5a3f54f4665db3d78b6e9f9e89eea4d"].into(),
					// 67kgfmY6zpw1PRYpj3D5RtkzVZnVvn49XHGyR4v9MEsRRyet
					hex!["46f630b3f79c588100dc0f69845633a830e01ea09eed4f1d01314a9bf33b9c16"].into(),
					// 67D6ecyNhnAzZqgRbxr3MdGnxB9Bw8VadMhjpLAYB3wf5Pq6
					hex!["2edf0fd8948ea642f135b314b1358c77ec6d0a4af83220b6ea18136e5ce36277"].into(),
					// 6AMsXyV1CYc3LMTk155JTDGEzbgVPvsX9aXp7VXz9heC3iuP
					hex!["ba44d2c00d9528c2d1fc51cef8ce8b9c3939928ecda8f404cdc46e3a2c090627"].into(),
				],
				vec![
					// 67nmVh57G9yo7sqiGLjgNNqtUd7H2CSESTyQgp5272aMibwS
					hex!["488ced7d199b4386081a52505962128da5a3f54f4665db3d78b6e9f9e89eea4d"].into(),
					// 67kgfmY6zpw1PRYpj3D5RtkzVZnVvn49XHGyR4v9MEsRRyet
					hex!["46f630b3f79c588100dc0f69845633a830e01ea09eed4f1d01314a9bf33b9c16"].into(),
					// 6A6VuGbeUwm3J2HqLduH7VFZTvrYQs8GuqzdhopLGN2JKMAe
					hex!["ae8b51cd0aa290645e593a4f54673ae62bab95791a137b943723bb6070533830"].into(),
					// 699YyPF2uA83zsFnQU4GCAvZXzucvyS5rx8LS9UrL9kEv8PP
					hex!["84a328f5f568d82ecd91861df7eae1065c1a2f1bcfec0950d4124e9363205b4a"].into(),
					// 669ocRxey7vxUJs1TTRWe31zwrpGr8B13zRfAHB6yhhfcMud
					hex!["001fbcefa8c96f3d2e236688da5485a0af67988b78d61ea952f461255d1f4267"].into(),
				],
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// Protocol ID
		Some("turing"),
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: TURING_RELAY_CHAIN.into(), // You MUST set this to the correct network!
			para_id: live_para_id,
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	para_id: ParaId,
	pallet_gates_closed: Vec<Vec<u8>>,
	vesting_schedule: Vec<(u64, Vec<(AccountId, Balance)>)>,
	general_councils: Vec<AccountId>,
	technical_memberships: Vec<AccountId>,
) -> turing_runtime::GenesisConfig {
	turing_runtime::GenesisConfig {
		system: turing_runtime::SystemConfig {
			code: turing_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: turing_runtime::BalancesConfig { balances: endowed_accounts },
		parachain_info: turing_runtime::ParachainInfoConfig { parachain_id: para_id },
		session: turing_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// Defaults to active collators from session pallet unless configured otherwise
		parachain_staking: Default::default(),
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		council: CouncilConfig { members: general_councils, phantom: Default::default() },
		democracy: Default::default(),
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: technical_memberships,
			phantom: Default::default(),
		},
		parachain_system: Default::default(),
		sudo: SudoConfig { key: Some(root_key) },
		treasury: Default::default(),
		valve: ValveConfig { start_with_valve_closed: false, closed_gates: pallet_gates_closed },
		vesting: VestingConfig { vesting_schedule },
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn validate_turing_allocation() {
		let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
		let initial_allocation: Vec<(AccountId, Balance)> =
			serde_json::from_slice(allocation_json).unwrap();
		const EXPECTED_ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
		validate_allocation(initial_allocation, EXPECTED_ALLOC_TOKENS_TOTAL, EXISTENTIAL_DEPOSIT);
	}

	#[test]
	fn validate_turing_vesting() {
		let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
		let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
			serde_json::from_slice(vesting_json).unwrap();

		let vested_tokens = 9_419_999_999_999_999_919;
		let vest_starting_time: u64 = 1651431600;
		let vest_ending_time: u64 = 1743534000;
		validate_vesting(
			initial_vesting,
			vested_tokens,
			EXISTENTIAL_DEPOSIT,
			vest_starting_time,
			vest_ending_time,
		);
	}

	#[test]
	fn validate_total_turing_tokens() {
		use crate::chain_spec::validate_total_tokens;
		let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
		let initial_allocation: Vec<(AccountId, Balance)> =
			serde_json::from_slice(allocation_json).unwrap();

		let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
		let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
			serde_json::from_slice(vesting_json).unwrap();

		let expected_vested_tokens = 9_419_999_999_999_999_919;
		let expected_allocated_tokens = DOLLAR * 58_000_000;
		let expected_total_tokens = expected_vested_tokens + expected_allocated_tokens;
		validate_total_tokens(initial_allocation, initial_vesting, expected_total_tokens);
	}
}