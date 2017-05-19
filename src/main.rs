use std::collections::BTreeSet;
use std::env::args;
use std::process;
use std::fs::File;
use std::io::Read;

fn usage() {
    println!("config-diff config config");
}

type BlocksVec = Vec<BTreeSet<String>>;

struct Config {
    original: String,
    blocks: BlocksVec,
}

fn read_config(path: &str) -> Config {
    let mut content = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut content))
        .expect(&format!("Error reading config: {}", path));

    let mut block: BTreeSet<String> = BTreeSet::new();
    let mut config: BlocksVec = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            if !block.is_empty() {
                config.push(block);
                block = BTreeSet::new();
            }
            continue;
        }

        // remove tabs and whitespaces
        let escaped_line = line.replace("\t", "").replace(" ", "");

        block.insert(escaped_line);
    }

    if !block.is_empty() {
        config.push(block);
    }

    Config {
        original: content,
        blocks: config,
    }
}

fn compare_configs(config0: &mut Config, config1: &mut Config) {
    while !config0.blocks.is_empty() && !config1.blocks.is_empty() {
        let search = config0.blocks.pop().unwrap().clone();

        let mut found = false;

        for i in 0..config1.blocks.len() {
            if config1.blocks.get(i) == Some(&search) {
                found = true;
                config1.blocks.remove(i);
                break;
            }
        }

        if !found {
            println!("First config: \n{}\n\n--------------------------------------",
                     config0.original);
            println!("Second config: \n{}\n\n--------------------------------------",
                     config1.original);
            panic!("Could not find a block in the second config: {:?}", search);
        }
    }

    if !config1.blocks.is_empty() {
        println!("First config: \n{}\n\n--------------------------------------",
                 config0.original);
        println!("Second config: \n{}\n\n--------------------------------------",
                 config1.original);
        panic!("The second config contains more blocks than the first config! Leftover: {:?}",
               config1.blocks);
    }
}

fn main() {
    if args().count() < 3 {
        usage();
        process::exit(1);
    }

    let mut config0 = read_config(&args().nth(1).unwrap());
    let mut config1 = read_config(&args().nth(2).unwrap());

    compare_configs(&mut config0, &mut config1);
}
