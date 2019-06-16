use std::io::Read;

fn main() {
    let mut handle = std::io::stdin();
    let mut conf = String::new();
    handle.read_to_string(&mut conf).unwrap();

    let conf = soo::config::read_config(&conf);
    println!("{:#?}", conf);
}
