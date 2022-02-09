use elabs_solc::Solc;

fn main() {
    let solc = Solc::new();
    let ver = solc.version();
    println!("{}", ver);
}
