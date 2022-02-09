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

use clap::{AppSettings, Parser};

/// CLI app structure.
#[derive(Parser)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

/// Subcommand structure.
#[derive(Parser)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub enum Subcommand {
    // Show the app license.
    #[clap(about = "Show the app license.")]
    License,
    // Show the app version.
    #[clap(about = "Show the app version.")]
    Version,
    // Update the app.
    #[clap(about = "Update the app.")]
    Update,
    // Init new project.
    #[clap(about = "Init new project.")]
    Init {
        // Path to the project directory.
        path: String,

        // Project license.
        // Default: MIT
        #[clap(long, short, default_value = "MIT")]
        license: String,
    },
    // Command to create new solidity contract or create deployment configuration file.
    // The command will have 2 subcommands:
    // 1. contract - create new solidity contract.
    // 2. deployment - create deployment configuration file.
    #[clap(about = "Create new solidity contract or create deployment configuration file.")]
    New {
        // Subcommand to create new solidity contract or create deployment configuration file.
        // Args required for subcommand.
        #[clap(subcommand)]
        subcommand: NewSubcommand,
    },
}

/// Subcommand for the `new` command.
#[derive(Parser)]
#[clap(setting = AppSettings::SubcommandRequired)]
pub enum NewSubcommand {
    // Create new solidity contract.
    #[clap(about = "Create new solidity contract.")]
    Contract {
        // Contracts name.
        // It cannot be empty.
        contracts: Vec<String>,
    },
    // Create deployment configuration file.
    #[clap(about = "Create deployment configuration file.")]
    Deployment {
        // Deployment name.
        // It cannot be empty.
        #[clap(long, short)]
        name: String,
    },
}
