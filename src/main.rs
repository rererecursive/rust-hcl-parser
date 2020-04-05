#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub hcl); // synthesized by LALRPOP

#[test]
fn test_single_attribute() {
    let text = r#"
variable hello "ok" {
  type = string
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_multiple_attributes() {
    let text = r#"
variable hello "ok" {
  type = string
  hello = there
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_collection_value() {
    let text = r#"
variable hello "ok" {
  type = string
  hello = there
  items = ["hello", 1, (3), ((apple))]
  objs = {apple = green}
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

// #[test]
// fn test4() {
//     let text = r#"
// variable hello "ok" {
//   type = string
//   text = <<-EOF
// }
// "#;
//     assert!(hcl::HclParser::new().parse(text).is_ok());
// }

#[test]
fn test_function_call() {
    let text = r#"
variable hello "ok" {
  once = rotate()
  twice = rotate(2)
  thrice = rotate(3, {apple = green}, translate(1))
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

fn main() {
    println!("Hello, world!");
}
