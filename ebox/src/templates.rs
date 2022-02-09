// Copyright (C) 2022 The Elabs Project Authors.
// This file is part of the Elabs library.
//
// The Elabs library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// The Elabs library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with The Elabs library.
// If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ebox config templates.
/// This struct will be used to generate the configuration file for the ebox.
/// It will be produce json data as shown below.
/// ```json
/// {
///    "name": "test",
///    "license": "MIT",
///    "compiler": {
///         "optimize": true,
///         "runs": 200
///    },
///    "networks": {
///         "dev": {
///             "name": "dev",
///             "host": "http://localhost:8545",
///         },
///    },
///    "key": ".private"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EboxConfig {
    /// name of the project.
    pub name: String,
    /// license of the project.
    pub license: String,
    /// compiler settings.
    pub compiler: Compiler,
    /// networks.
    pub networks: HashMap<String, Network>,
    /// key file.
    pub key: String,
}

/// Compiler settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct Compiler {
    /// optimize the code.
    pub optimize: bool,
    /// number of runs.
    pub runs: u64,
}

/// Network settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    /// name of the network.
    pub name: String,
    /// host of the network.
    pub host: String,
}

/// Ebox config generator.
/// This will create confif from EboxConfig and return the json string.
/// # Arguments
/// * `name` - name of the project.
/// * `license` - license of the project.
/// # Returns
/// * `String` - json string.
/// # Errors
/// * `serde_json::Error` - if serde_json fails.
pub fn generate_config(name: &str, license: &str) -> Result<String, serde_json::Error> {
    let config = EboxConfig {
        name: name.to_string(),
        license: license.to_string(),
        compiler: Compiler {
            optimize: true,
            runs: 200,
        },
        networks: HashMap::new(),
        key: ".private".to_string(),
    };
    let data = serde_json::to_string(&config)?;
    Ok(data)
}

/// Ebox config decoder.
/// This will decode the json string and return the EboxConfig struct.
/// # Arguments
/// * `config` - json string.
/// # Returns
/// * `EboxConfig` - EboxConfig struct.
/// # Errors
/// * `serde_json::Error` - error while decoding.
pub fn decode(config: &str) -> Result<EboxConfig, serde_json::Error> {
    serde_json::from_str(config)
}

/// Generate the solidity code.
/// This will generate the solidity code as shown below.
/// ```solidity
/// // SPDX-License-Identifier: {license}
///
/// pragma solidity ^0.8.7;
///
/// contract {name} {
///    function test() public {
///         // code
///     }
/// }
/// ```
/// # Arguments
/// * `name` - name of the contract.
/// * `license` - license of the project.
/// # Returns
/// * `String` - solidity code.
pub fn generate_solidity(name: &str, license: &str) -> String {
    let mut solidity = String::new();
    solidity.push_str(&format!("// SPDX-License-Identifier: {}\n\n", license));
    solidity.push_str("pragma solidity ^0.8.7;\n\n");
    solidity.push_str(&format!("contract {} {{\n", name));
    solidity.push_str("    function test() public {");
    solidity.push_str("    }\n");
    solidity.push_str("}\n");
    solidity
}
