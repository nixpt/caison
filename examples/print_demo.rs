//! Print a CAISON file back out — `cargo run --example print_demo -- file.caison`
use caison::CaisonParser;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: print_demo <file.caison>");
    let src = std::fs::read_to_string(&path).expect("read");
    let doc = CaisonParser::new(&src).parse().expect("parse");
    print!("{}", doc.print());
}
