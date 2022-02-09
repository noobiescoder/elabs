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

use crate::{templates, utils};

/// Function for initiating a new ethereum contract project.
/// # Arguments
/// * `path` - The path to the new project.
/// * `license` - The license to use for the project.
/// # Returns
/// * `Ok(())` if the project was created successfully.
/// * `Err(String)` if the project could not be created.
pub fn init_project(path: &str, license: &str) -> Result<(), String> {
    // check if directory exist.
    // if exist check if directory doesn't contains `ebox.json` file.
    // if `ebox.json` file exist return error.
    // if not, create it
    if utils::directory_exists(path) {
        if utils::file_exists(format!("{}/ebox.json", path).as_str()) {
            return Err("Project already exist".to_string());
        }
    } else {
        utils::create_directory(path)?;
    }

    // generate the config file from the template.
    let config = templates::generate_config(path, license).map_err(|e| format!("{:?}", e))?;

    // write the config file to the project.
    utils::write_file(format!("{}/ebox.json", path).as_str(), &config)?;

    Ok(())
}

/// Function for creating a new ethereum contract.
/// # Arguments
/// * `contracts` - List of contracts to create.
/// # Returns
/// * `Ok(())` if the contract was created successfully.
/// * `Err(String)` if the contract could not be created.
pub fn new_contracts(contracts: Vec<String>) -> Result<(), String> {
    let config_file = utils::read_file("ebox.json").map_err(|e| format!("{:?}", e))?;
    let config = templates::decode(&config_file).map_err(|e| format!("{:?}", e))?;

    if !utils::directory_exists("contracts") {
        utils::create_directory("contracts")?;
    }

    for contract in contracts {
        let contract_path = format!("contracts/{}.sol", contract);
        let contract_content = templates::generate_solidity(&contract, &config.license);
        let _ = utils::write_file(&contract_path, &contract_content)?;

        let log = ansi_term::Colour::Green.paint(format!("Contract {} created", contract));
        println!("{}", log);
    }

    Ok(())
}
