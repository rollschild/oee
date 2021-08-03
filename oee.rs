extern crate rand;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;

const CYPHERBYTES: usize = 32;

fn save(fname: &str, buffer: &[u8]) {
    let mut f = match File::create(fname) {
        Ok(file) => file,
        Err(e) => { panic!("error creating file: {}", e.to_string()); },
    };

    match f.write(buffer) {
        Ok(_) => return,
        Err(e) => { panic!("error writing file: {}", e.to_string()); },
    };
}

fn main() {
    let mut inplace: bool = false;

    let args: Vec<_> = env::args().collect();
    if (args.len() != 2 && args.len() != 3) ||
       (args.len() == 3 && args[1].as_str() != "-i") {
        println!("usage: {} [-i] file-to-encrypt", args[0]);
        return
    }

    if args.len() == 3 && args[1].as_str() == "-i" {
	inplace = true;
    }

    let mut csprng = match rand::OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("cyber attack detected:  {}", e)
    };

    let mut cyphertext = [0u8; CYPHERBYTES];
    csprng.fill_bytes(&mut cyphertext);

    if inplace {
        save(args[2].as_str(), &cyphertext);
    } else {
        for x in &cyphertext { print!("{:02x}", *x); }
        println!("");
    }
}
