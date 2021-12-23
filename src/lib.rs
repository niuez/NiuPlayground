use wasm_bindgen::prelude::*;

extern crate log;
extern crate niu;

//use crate::trans::Transpile;

fn type_check(prog: &str) -> Result<String, String> {
    let std_ope = include_str!("../lib/std/opes.niu");
    let std_u64 = include_str!("../lib/std/u64.niu");
    let std_i64 = include_str!("../lib/std/i64.niu");
    let std_f64 = include_str!("../lib/std/f64.niu");
    let std_bool = include_str!("../lib/std/bool.niu");
    let std_tuple = include_str!("../lib/std/tuple.niu");
    let prog = 
        std_ope.to_string() +
        std_u64 +
        std_i64 + 
        std_f64 + 
        std_bool +
        std_tuple +
        prog;
    let (s, (_, t)) = niu::full_content::parse_full_content(&prog, "main.niu").map_err(|e| format!("{:?}", e))?;
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
