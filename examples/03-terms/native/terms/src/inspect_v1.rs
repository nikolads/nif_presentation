use rustler::{Encoder, Env, Term};
use super::{atoms, term_to_string};

#[rustler::nif]
pub fn inspect<'a>(env: Env<'a>, term: Term<'a>) -> Term<'a> {
    match term_to_string(term) {
        Some(string) => (atoms::ok(), format!("got {string}")).encode(env),
        None => (atoms::error(), atoms::unknown_term()).encode(env),
    }
}
