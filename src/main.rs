#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub hcl); // synthesized by LALRPOP
pub mod ast;

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

#[test]
fn test_for_expr() {
    let text = r#"
variable hello "ok" {
  once = [for a in items(): a.key]
  twice = [for a,b in items(): a.key if hasvalue(a)]
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_operation_binary() {
    let text = r#"
variable hello "ok" {
  once = one || two
  twice = one && (two == three) >= four
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_operation_unary() {
    let text = r#"
variable hello "ok" {
  once = -1
  twice = !stopped("now")
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_conditional() {
    let text = r#"
variable hello "ok" {
  once = true ? one : two
  twice = (1 == 0) ? fn(1) : fn(0)
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_template_interpolation() {
    let text = r#"
variable hello "ok" {
  once = ${var.hello}
  twice = ${some.object.attribute}
  thrice = ${~ some.object.attribute ~}
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}

#[test]
fn test_simple() {
    let text = r#"
variable "ok" {
  hi = yes
}
"#;
    assert!(hcl::HclParser::new().parse(text).is_ok());
}


fn main() {
    let text = r#"
variable hello "ok" {
  once = rotate()
  twice = rotate(2)
  thrice = rotate(3, {apple = green}, translate(1))
}

data hello "no" {
  frice = "yum"
}

ident hello "no" {
  frice = "yum"
}
"#;
    hcl::HclParser::new().parse(text);
    println!("Hello, world!");
}
