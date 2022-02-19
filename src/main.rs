use std::env::args;
use std::process::exit;
fn main() {

    let args_joined = args().skip(1).collect::<Vec<_>>().join(" ");
    // println!("Joined form: {:?}", args_joined);
    
    let chars = args_joined.chars().collect::<Vec<_>>();
    
    if chars == vec![] || args().skip(1).any(|arg| arg == "--help") {
        println!(include_str!("../issiz.help"));
        exit(1);
    }
    for i in 0..=(chars.len()) {
        println!("{}",String::from_iter((&chars[0..i]).iter()));
    }

    for i in (0..(chars.len())).rev() {
        println!("{}", String::from_iter((&chars[0..i]).iter()));
    }
}
