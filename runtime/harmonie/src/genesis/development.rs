// This file is part of Allfeat.

// Copyright (C) 2022-2024 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::{authority_keys_from_seed, genesis};
use shared_runtime::genesis_utils::development_account;
use sp_std::vec;

/// Return the development genesis config.
pub fn development_config_genesis() -> serde_json::Value {
	let accounts = development_account();

	genesis(vec![authority_keys_from_seed("Alice")], accounts[0], accounts)
}