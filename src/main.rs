//! TeX Stripper
//!
//! A simple program to enable specific environments to be stripped from large TeX documents and placed in different documents. For more detail see README.md. 
//!
//! -------------------------------------
//! Copyright 2023 J D Pickering
//!
//! This file is part of tex_stripper.
//!
//! tex_stripper is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! tex_stripper is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with Foobar. If not, see <https://www.gnu.org/licenses/>.
//! --------------------------------------


use std::process;
use clap::Parser;
use tex_stripper::Cli;




fn main() {
    // read the command line arguments and collect into a vector
    let args = Cli::parse();
    
    println!("Reading from tex file: {}", args.input);  
    println!("Writing to file: {}", args.output);
    println!("Ignoring any of the following environments: {}", args.boxes_ignore.join(", "));
    // if running produces an error, then do this, otherwise run program
    if let Err(e) = tex_stripper::run(args) {
        println!("Application error: {e}");
        process::exit(1);
        }
    }


