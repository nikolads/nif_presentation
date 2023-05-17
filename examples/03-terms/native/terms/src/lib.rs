use rustler::{Term, TermType};

mod get_foobar;
mod inspect_v1;
mod inspect_v2;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unknown_term,
        foo,
        bar,
        baz,
    }
}

fn term_to_string(term: Term) -> Option<String> {
    match term.get_type() {
        TermType::Atom => {
            let atom_string = term.atom_to_string().unwrap();
            Some(format!(":{atom_string}"))
        }
        TermType::Binary => {
            let bytes = term.into_binary().unwrap().as_slice();
            let str_slice = std::str::from_utf8(bytes).ok()?;

            Some(format!("\"{str_slice}\""))
        }
        TermType::List | TermType::EmptyList => {
            let items_strings = term
                .into_list_iterator()
                .unwrap()
                .map(|item| term_to_string(item))
                .collect::<Option<Vec<String>>>()?;

            let items_joined = items_strings.join(", ");
            Some(format!("[{items_joined}]"))
        }
        _ => None,
    }
}

rustler::init!(
    "Elixir.NifTerms",
    [inspect_v1::inspect, inspect_v2::inspect_v2, get_foobar::get_foobar]
);
