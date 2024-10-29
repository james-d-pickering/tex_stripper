//! # TeX Stripper
//!
//! `tex_stripper' is a simple program that allows specific environments from within large TeX documents to be stripped from that document and placed into a new TeX file. 
//!
//! The initial intended use case was to strip figure, table, and block environments from a set of text-heavy lecture notes and automatically insert them into a beamer presentation, thus generating a corresponding set of slides that could be used when delivering the lectures.   
//!
//! A decision was made to do this in Rust because I wanted an excuse to learn basic Rust. Inefficient, non-idiotmatic, and probably really stupid code is below - however, it works.
//! 
//! See README for more info.

//! ----------------------------------------------
//! Copyright 2023 J D Pickering

//! This file is part of tex_stripper.
//!
//! tex_stripper  is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! tex_stripper is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with Foobar. If not, see <https://www.gnu.org/licenses/>.
//! ----------------------------------------------

// Import necessary libraries/objects/macros
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::io::Write;
use std::path;
use std::vec::Vec;
use clap::Parser;
 



// main run function. takes cmd line args (Cli) and reads, parses, strips files
pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    

    println!("Reading from: {}", args.input);
    // define a new vector for main file
    let mut vec = Vec::new();
    // define vectors to hold stuff from input file
    let mut begin_block_vec = Vec::new();
    let mut end_block_vec = Vec::new();
    let mut header_vec = Vec::new();
    let mut tail_vec = Vec::new();
    // define some boolean flags and counter (clunky but whatever)
    let mut in_block = false;
    let mut ignore_block = false;
    let mut block_counter = 0;
    let mut input_counter = 0;

    // how you want to define the environment around stripped stuff, from input file
    // input file is ordered as begin block, end block, document tail, document header
    if let Ok(content) = read_lines(&args.inputfile) {
        'inputloop: for line in content {
            if let Ok(text) = line { 
                if text.trim().starts_with("%") { 
                    input_counter = input_counter + 1;
                    continue 'inputloop }
                if input_counter == 1 { 
                    // if we are in the first input file block
                    begin_block_vec.push(text.clone());
                    }
                else if input_counter == 2 {
                    // second input file block
                    end_block_vec.push(text.clone());
                    }
                else if input_counter == 3 {
                    // third input file block
                    tail_vec.push(text.clone());
                    }
                else if input_counter == 4 {
                    // fourth input file block
                    header_vec.push(text.clone());
                    }
                else { panic!("Too many things in input file!") }
                }
            }
        }
        
    vec.append(&mut header_vec);

    if let Ok(contents) = read_lines(&args.input) {
         'lineloop: for line in contents {
             // if the line is there (i.e. not empty) then carry on
             if let Ok(ip) = line {
                // ignore the begin document
                if ip.trim().starts_with("\\begin{document}") { continue 'lineloop }
                // look for other begins
                if ip.trim().starts_with("\\begin") {
                    // if you are not already in an environment, and are now in one that is
                    // on the ignore list:
                    if in_block == false {
                        for texbox in &args.boxes_ignore { if ip.contains(texbox) { 
                            in_block = true;
                            block_counter = block_counter + 1;
                            ignore_block = true;
                            continue 'lineloop }} // skip to next line
                        // if you don't want to ignore this environment, push header output vec
                        if ignore_block == false { vec.append(&mut begin_block_vec.clone());}
                        block_counter = block_counter +  1;
                        in_block = true;
                        // now push block contents to the output vec
                        if ignore_block == false { vec.push(ip.clone()); }
                    }
                    // if you are already in an environment
                    else {
                        // if it isn't one to ignore, then push line
                        if ignore_block == false {vec.push(ip.clone());}
                        block_counter = block_counter + 1
                    }
                    
                }

                else { 
                    // if line  doesnt start with begin
                    if in_block {
                        // if in the block, push the next line
                        if ignore_block == false {vec.push(ip.clone());}
                        // and if it starts with \end change the bool flag
                        if ip.trim().starts_with("\\end") {
                            block_counter = block_counter - 1; 
                            if block_counter == 0 { 
                            // push the tail if the block is finished and reset flags
                                if ignore_block == false {vec.append(&mut end_block_vec.clone());} 
                                in_block = false;
                                ignore_block=false;}
                        }
                        else {}
                    }
                    else {}
                }
            }
        }
    }

    // push the tail of the whole file
    vec.append(&mut tail_vec);
    println!("Writing to: {}", args.output);
    // open the output file for writing
    let path = path::Path::new(&args.output);
    let mut writefile = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create file because {}", why),
        Ok(writefile) => writefile
    };
    // write the stripped envrionments to a new tex file
    write_lines(&mut writefile, &vec);

    Ok(println!("Successfully stripped from {} and written to {}", &args.input, args.output))
    
}

pub fn write_lines(writefile: &mut fs::File, data: &Vec<String>) -> () {
    // open a file and write the data to it
    let mut file = io::LineWriter::new(writefile);
    // change this so it handles the error properly later
    // join the vec<string> into a series of strings and then convert to bytes for writing
    let _ = file.write_all(data.join("\n").as_bytes());
}


pub fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<path::Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
    }

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,

    #[arg(short, long, default_value = None, num_args=1.., value_delimiter=' ')]
    pub boxes_ignore: Vec<String>,

    #[arg(short ='f', long, default_value = None)]
    pub inputfile: String,
}

