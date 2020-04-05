#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
	assert!(calculator1::TermParser::new().parse("((22))").is_ok());
}

fn main() {
    println!("{:?}", calculator1::TermParser::new().parse("((22))").unwrap());
    println!("Hello, world!");
}
