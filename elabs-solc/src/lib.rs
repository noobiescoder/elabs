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

//! # Elabs
//! Elabs-solc is a wrapper around the Solidity compiler.
//! It is designed to be used as a library, and not as a command line tool.
//! It will wrap `solc` cli tools, and provide a simple interface
//! to compile solidity contracts.
//! ## Example
//! ```rust
//! use elabs_solc::Solc;
//!
//! fn main() {
//!    let solc = Solc::new();
//!    let input_path = "contracts/SimpleStorage.sol";
//!    let output_path = "artifacts";
//!    let compile = solc.compile(input_path, output_path);
//!    let result = compile.unwrap();
//!    println!("{}", result);
//! }
//! ```

use std::process::Command;

/// The solc struct.
/// It is a wrapper around the solc compiler.
pub struct Solc(String);

impl Solc {
    /// Create a new solc wrapper.
    /// # Arguments
    /// * `binary` - The binary to use. eg. `solc`.
    /// # Returns
    /// * `Solc` - The solc wrapper.
    /// # TODO
    /// * Add error handling.
    /// * Add support for other compilers. eg `solcjs`.
    pub fn new() -> Solc {
        Solc("solc".to_string())
    }

    /// Parse version number.
    /// # Arguments
    /// * `version` - The version string.
    /// # Returns
    /// * `String` - The version number.
    pub fn parse_version(version: &str) -> String {
        version
            .replace("Version: ", "")
            .split("+")
            .next()
            .unwrap()
            .to_string()
    }

    /// Get the solc version.
    /// # Returns
    /// * `String` - The solc version.
    pub fn version(&self) -> String {
        let mut cmd = Command::new(&self.0);
        cmd.arg("--version");
        let output = cmd.output().unwrap();
        let out_str = String::from_utf8(output.stdout).unwrap();
        let out_vec = out_str.split("\n").collect::<Vec<&str>>();
        if out_vec.len() > 1 {
            Solc::parse_version(out_vec[1])
        } else {
            Solc::parse_version(out_vec[0])
        }
    }

    /// Compile solidity code.
    /// # Arguments
    /// * `input_path` - The path to the solidity file.
    /// * `out_path` - The path to the output file.
    /// * `opts` - Optional arguments.
    /// # Returns
    /// * `Ok(String)` - The compiled contract.
    /// * `Err(String)` - The error message.
    pub fn compile(
        &self,
        input_path: &str,
        out_path: &str,
        opts: Option<&str>,
    ) -> Result<String, String> {
        let args = vec!["--bin", "--abi", "--overwrite"];

        let cmd = Command::new(&self.0)
            .args(args)
            .args(opts)
            .arg("--output-dir")
            .arg(out_path)
            .arg(input_path)
            .output();

        match cmd {
            Err(err) => Err(format!("{}", err)),
            Ok(res) => {
                // check if stderr was empty, if not return it as error.
                if res.stderr.len() > 0 {
                    Err(String::from_utf8(res.stderr).unwrap())
                } else {
                    Ok(String::from_utf8(res.stdout).unwrap())
                }
            }
        }
    }
}
