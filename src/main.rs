use std::env::args;

fn main() {
    let txt = args().skip(1);
    let mut txt2 = String::new();
    for i in txt {
        txt2.push_str(i.as_str());
        txt2.push(' ');
    }
    let _ = txt2.pop();

    for i in 0..=(txt2.len()) {
        println!("{}", &txt2[0..i]);
    }
    
    for i in (0..(txt2.len())).rev() {
        println!("{}", &txt2[0..i]);
    }
}
