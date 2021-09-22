use wasm_bindgen::prelude::*;

extern crate log;
extern crate niu;

//use crate::trans::Transpile;

const NIU_STD: &'static str = "
#include <type_traits>
trait Neg {
  type Output;
  fn neg(a: Self) -> Self#Neg::Output;
}
trait Not {
  type Output;
  fn not(a: Self) -> Self#Not::Output;
}

trait BitOr<Arg> {
  type Output;
  fn bit_or(a: Self, b: Arg) -> Self#BitOr<Arg>::Output;
}

trait BitXor<Arg> {
  type Output;
  fn bit_xor(a: Self, b: Arg) -> Self#BitXor<Arg>::Output;
}

trait BitAnd<Arg> {
  type Output;
  fn bit_and(a: Self, b: Arg) -> Self#BitAnd<Arg>::Output;
}

trait Shl<Arg> {
  type Output;
  fn shl(a: Self, b: Arg) -> Self#Shl<Arg>::Output;
}

trait Shr<Arg> {
  type Output;
  fn shr(a: Self, b: Arg) -> Self#Shr<Arg>::Output;
}

trait Add<Arg> {
  type Output;
  fn add(a: Self, b: Arg) -> Self#Add<Arg>::Output;
}

trait Sub<Arg> {
  type Output;
  fn sub(a: Self, b: Arg) -> Self#Sub<Arg>::Output;
}

trait Mul<Arg> {
  type Output;
  fn mul(a: Self, b: Arg) -> Self#Mul<Arg>::Output;
}

trait Div<Arg> {
  type Output;
  fn div(a: Self, b: Arg) -> Self#Div<Arg>::Output;
}

trait Rem<Arg> {
  type Output;
  fn rem(a: Self, b: Arg) -> Self#Rem<Arg>::Output;
}

trait Index {
  type Output;
  type Arg;
  fn index(self: &Self, i: Self#Index::Arg) -> &Self#Index::Output;
}

trait IndexMut where Self: Index {
  fn index_mut(self: &mut Self, i: Self#Index::Arg) -> &mut Self#Index::Output;
}
impl BitOr<u64> for u64 {
  type Output = u64;
  fn bit_or(a: Self, b: u64) -> u64 $${a | b}$$
}
impl BitXor<u64> for u64 {
  type Output = u64;
  fn bit_xor(a: Self, b: u64) -> u64 $${a ^ b}$$
}
impl BitAnd<u64> for u64 {
  type Output = u64;
  fn bit_and(a: Self, b: u64) -> u64 $${a & b}$$
}
impl Shl<u64> for u64 {
  type Output = u64;
  fn shl(a: Self, b: u64) -> u64 $${a << b}$$
}
impl Shr<u64> for u64 {
  type Output = u64;
  fn shr(a: Self, b: u64) -> u64 $${a >> b}$$
}
impl Add<u64> for u64 {
  type Output = u64;
  fn add(a: Self, b: u64) -> u64 $${a + b}$$
}
impl Sub<u64> for u64 {
  type Output = u64;
  fn sub(a: Self, b: u64) -> u64 $${a - b}$$
}
impl Mul<u64> for u64 {
  type Output = u64;
  fn mul(a: Self, b: u64) -> u64 $${a * b}$$
}
impl Div<u64> for u64 {
  type Output = u64;
  fn div(a: Self, b: u64) -> u64 $${a / b}$$
}
impl Rem<u64> for u64 {
  type Output = u64;
  fn rem(a: Self, b: u64) -> u64 $${a % b}$$
}
impl BitOr<i64> for i64 {
  type Output = i64;
  fn bit_or(a: Self, b: i64) -> i64 $${a | b}$$
}
impl BitXor<i64> for i64 {
  type Output = i64;
  fn bit_xor(a: Self, b: i64) -> i64 $${a ^ b}$$
}
impl BitAnd<i64> for i64 {
  type Output = i64;
  fn bit_and(a: Self, b: i64) -> i64 $${a & b}$$
}
impl Shl<i64> for i64 {
  type Output = i64;
  fn shl(a: Self, b: i64) -> i64 $${a << b}$$
}
impl Shr<i64> for i64 {
  type Output = i64;
  fn shr(a: Self, b: i64) -> i64 $${a >> b}$$
}
impl Add<i64> for i64 {
  type Output = i64;
  fn add(a: Self, b: i64) -> i64 $${a + b}$$
}
impl Sub<i64> for i64 {
  type Output = i64;
  fn sub(a: Self, b: i64) -> i64 $${a - b}$$
}
impl Mul<i64> for i64 {
  type Output = i64;
  fn mul(a: Self, b: i64) -> i64 $${a * b}$$
}
impl Div<i64> for i64 {
  type Output = i64;
  fn div(a: Self, b: i64) -> i64 $${a / b}$$
}
impl Rem<i64> for i64 {
  type Output = i64;
  fn rem(a: Self, b: i64) -> i64 $${a % b}$$
}
impl Neg for i64 {
  type Output = i64;
  fn neg(a: Self) -> i64 $${-a}$$
}
impl Not for bool {
  type Output = bool;
  fn not(a: Self) -> bool $${!a}$$
}
impl Add<f64> for f64 {
  type Output = f64;
  fn add(a: Self, b: f64) -> f64 $${a + b}$$
}
impl Sub<f64> for f64 {
  type Output = f64;
  fn sub(a: Self, b: f64) -> f64 $${a - b}$$
}
impl Mul<f64> for f64 {
  type Output = f64;
  fn mul(a: Self, b: f64) -> f64 $${a * b}$$
}
impl Div<f64> for f64 {
  type Output = f64;
  fn div(a: Self, b: f64) -> f64 $${a / b}$$
}
";

fn type_check(prog: &str) -> Result<String, String> {
    let prog = NIU_STD.to_string() + prog;
    let (s, (_, mut t)) = niu::full_content::parse_full_content(&prog).map_err(|e| format!("{:?}", e))?;
    if s != "" {
        Err(format!("parse error\n{}", s))
    }
    else {
        let mut ta = t.type_check()?;
        t.mut_check(&ta)?;
        Ok(t.transpile(&mut ta))
    }
}

#[wasm_bindgen]
pub fn greet(prog: &str) -> String {
    //env_logger::init();
    match type_check(prog) {
        Ok(prog) => prog,
        Err(err) => err,
    }
}
