mod core_functional;
use core_functional::reader::*;

const filepath: &str = "/Users/blcklptn/Documents/Datalyzer/src/tests/testfiles/test.csv";

fn main() {
    let data = Csv {};
    let content = Csv.simply_reader(filepath.to_string());
}
