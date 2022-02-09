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

use std::process;

use clap::Parser;
use ebox::cli::{NewSubcommand, Subcommand};

fn main() {
    let apps = ebox::cli::Cli::parse();

    match &apps.subcommand {
        Subcommand::License => println!("{}", include_str!("../LICENSE")),

        Subcommand::Init { path, license } => match ebox::actions::init_project(path, license) {
            Ok(_) => {
                let log = ansi_term::Color::Green.paint("Project initialized");
                println!("{}", log);
            }
            Err(e) => {
                let log = ansi_term::Color::Red.paint(format!("{}", e));
                println!("{}", log);
                process::exit(1);
            }
        },

        Subcommand::Update => {
            println!("Updating");
        }

        Subcommand::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }

        Subcommand::New { subcommand } => match subcommand {
            NewSubcommand::Contract { contracts } => {
                let contract = ebox::actions::new_contracts(contracts.to_owned());
                if contract.is_err() {
                    let log = ansi_term::Color::Red.paint(format!("{}", contract.err().unwrap()));
                    println!("{}", log);
                    process::exit(1);
                }
            }

            NewSubcommand::Deployment { name } => {
                println!("Creating deployment {}", name);
            }
        },
    }
}
