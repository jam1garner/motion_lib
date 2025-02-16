use motion_lib;
use serde_yaml::{from_str, to_string};
use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

enum Mode {
    Disasm {
        file: String,
    },
    Asm {
        file: String,
    },
    Patch {
        file: String,
        patch: String,
    },
    Compare {
        a: String,
        b: String,
    },
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    let mode: Mode;
    let mut labelname = String::default();
    let mut outname = String::default();

    if len <= 1 {
        print_help_text();
        return;
    }

    let mut arg_index: usize;
    match args[1].as_ref() {
        "-h" => {
            print_help_text();
            return;
        }
        "-d" => {
            if len > 2 {
                mode = Mode::Disasm {
                    file: String::from(&args[2])
                };
            } else {
                println!("missing 'FILE' arg for disassembly");
                return;
            }
            arg_index = 3;
        }
        "-a" => {
            if len > 2 {
                mode = Mode::Asm {
                    file: String::from(&args[2])
                }
            } else {
                println!("missing 'FILE' arg for assembly");
                return;
            }
            arg_index = 3;
        }
        "-p" => {
            if len > 3 {
                mode = Mode::Patch {
                    file: String::from(&args[2]),
                    patch: String::from(&args[3])
                }
            } else {
                println!("missing 'FILE' or 'PATCH' arg for patching");
                return;
            }
            arg_index = 4;
        }
        "-c" => {
            if len > 3 {
                mode = Mode::Compare {
                    a: String::from(&args[2]),
                    b: String::from(&args[3])
                }
            } else {
                println!("missing 'A' or 'B' arg for comparison");
                return;
            }
            arg_index = 4;
        }
        _ => {
            println!("Unrecognized mode: '{}'", &args[1]);
            return;
        },
    }

    while arg_index < len {
        match args[arg_index].as_ref() {
            "-l" => {
                arg_index += 1;
                if arg_index < len {
                    labelname = String::from(&args[arg_index]);
                } else {
                    println!("missing 'FILE' arg for labels");
                }
            }
            "-o" => {
                arg_index += 1;
                if arg_index < len {
                    outname = String::from(&args[arg_index]);
                } else {
                    println!("missing 'OUTNAME' arg for output name");
                }
            },
            _ => {

            }
        }
        arg_index += 1;
    }

    if labelname.len() > 0 {
        if let Err(e) = motion_lib::hash40::load_labels(&labelname) {
            println!("Error loading labels: {}", e);
        }
    }

    match mode {
        Mode::Disasm {file} => {
            let o = if outname.len() > 0 {
                &outname
            } else {
                "out.yml"
            };

            match convert_to_yaml(&file, o) {
                Ok(_) => {}
                Err(y) => {
                    let e: &Error = y.borrow();
                    println!("ERROR: {}", e);
                }
            }
        }
        Mode::Asm {file} => {
            let o = if outname.len() > 0 {
                &outname
            } else {
                "out.bin"
            };

            match convert_to_bin(&file, o) {
                Ok(_) => {}
                Err(y) => {
                    let e: &Error = y.borrow();
                    println!("ERROR: {}", e);
                }
            }
        }
        Mode::Patch {file, patch} => {

        }
        Mode::Compare {a, b} => {

        }
    }
}

fn print_help_text() {
    println!("Args: [MODE] [OTHER]");
    println!("MODE:");
    println!("  -h (print help)");
    println!("  -d (disassemble) <FILE>");
    println!("  -a (assemble)    <FILE>");
    println!("  -p (patch)       <FILE> <PATCH>");
    println!("  -c (compare)     <A> <B>");
    println!("OTHER:");
    println!("  -l (label)       <FILE>");
    println!("  -o (out)         <OUTNAME>");
}

fn convert_to_yaml(i: &str, o: &str) -> Result<(), Box<Error>> {
    match motion_lib::open(i) {
        Ok(x) => {
            let mut f = File::create(o)?;
            let pretty = to_string(&x)?;
            f.write_all(pretty.as_bytes())?;
            Ok(())
        }
        Err(y) => Err(Box::new(y)),
    }
}

fn convert_to_bin(i: &str, o: &str) -> Result<(), Box<Error>> {
    let mut f = File::open(i)?;
    let mut s: String = String::default();
    f.read_to_string(&mut s)?;
    match from_str::<motion_lib::mlist::MList>(&s) {
        Ok(x) => match motion_lib::save(o, &x) {
            Ok(_) => Ok(()),
            Err(y) => Err(Box::new(y)),
        },
        Err(y) => Err(Box::new(y)),
    }
}
