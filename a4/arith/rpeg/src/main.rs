use std::env;
use rpeg::codec::{compress};


fn main() {
    let args: Vec<String> = env::args().collect(); let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap(); 
    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => compress(Some(filename)),
        _ => Ok({
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        })
    }.expect("REASON")
}