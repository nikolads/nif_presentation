use rustler::{Env, Term, NifResult, Encoder};
use std::thread;
use std::time::Duration;

use crate::atoms;

pub fn dirty_nif<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    thread::sleep(Duration::from_secs(5));
    Ok(atoms::work().encode(env))
}
