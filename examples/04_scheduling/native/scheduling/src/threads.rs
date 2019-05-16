use rustler::{Env, Term, NifResult, Encoder};
use rustler::env::OwnedEnv;
use std::thread;

use crate::atoms;

pub fn echo_from_thread<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let self_pid = env.pid();
    let term = args[0];

    let thread_env = OwnedEnv::new();
    let saved_term = thread_env.save(term);

    thread::spawn(move || {
        thread_env.run(|env| {
            let term = saved_term.load(env);

            // do work

            env.send(&self_pid, term);
        });
    });

    Ok(atoms::nil().encode(env))
}

