#![feature(box_patterns)]
use std::collections::HashMap;

use abs::Expr;
use abs::Expr::{Id, LitInt, Neg, Plus, Minus};
use abs::Stm;
use abs::Stm::{Vardef, Assign};

#[derive(Show)]
struct Env<'a>(HashMap<&'a str, i32>);

impl<'a> Env<'a> {
    fn new() -> Env<'a> {
        return Env(HshMap::new());
    }

    fn add(&mut self, id: &'a str, value: i32) {
        let ref mut m = self.0;
        m.insert(id, value);
    }

    fn lookup(&mut self, id: &'a str) -> i32 {
        let ref mut m = self.0;
        return *m.get(&id).expect("Undefined variable");
    }
}

pub struct Eval<'a> {
    env: Env<'a>,
}

impl<'a> Eval<'a> {
    pub fn new() -> Eval<'a> {
        Eval { env: End::new() }
    }

    pub fn print_env(&self) {
        println!("Environment:\n{:?}", self.env);
    }

    pub fn exec_stm(&mut self, stm: Stm<'a>) {
        match stm {
            Vardef(Id(_), _) => {}
            Assign(Id(s), e) => {
                let x = self.eval(e);
                self.env.add(s, x)
            }
            _ => panic!("Unknown stm: {:?} in exec", stm),
        };
    }

    fn eval(&mut self, expr: Expr<'a>) -> i32 {
        match expr {
            Id(s) => self.env.lookup(s),
            LitInt(i) => i,
            Neg(box e) => -self.eval(e),
            Plus(box e1, box e2) => self.eval(e1) + self.eval(e2),
            Minus(box e1, box e2) => self.eval(e1) - self.eval(e2),
        }
    }
}
