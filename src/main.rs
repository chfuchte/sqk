pub(crate) mod driver;
mod ui;
mod parser;

use parser::args::parse_args;

fn main() {
    let _args = parse_args();
}
