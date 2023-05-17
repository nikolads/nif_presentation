use rustler::{Atom, Term};
use super::{atoms, term_to_string};

#[rustler::nif]
pub fn inspect_v2<'a>(term: Term<'a>) -> Result<String, Atom> {
    match term_to_string(term) {
        Some(string) => Ok(format!("got {string}")),
        None => Err(atoms::unknown_term()),
    }
}
