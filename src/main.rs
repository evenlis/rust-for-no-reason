/// I'm now the king of documentation
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        println!("{}", arg);
    }
}
