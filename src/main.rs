#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub hcl); // synthesized by LALRPOP

#[test]
fn hcl() {
    let ml = r#"
variable "hello" {
  type = string
}
"#;
    assert!(hcl::StructureParser::new().parse(ml).is_ok());
}

fn main() {
    println!("Hello, world!");
}
