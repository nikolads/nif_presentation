use rustler::{Atom, NifTaggedEnum};
use super::atoms;

#[derive(NifTaggedEnum)]
pub enum FooBar {
    Foo,
    Bar(String),
    Baz { a: i32, b: i32 },
}

#[rustler::nif]
pub fn get_foobar(kind: Atom) -> Result<FooBar, rustler::Error> {
    match kind {
        _ if kind == atoms::foo() => Ok(FooBar::Foo),
        _ if kind == atoms::bar() => Ok(FooBar::Bar("bar".to_string())),
        _ if kind == atoms::baz() => Ok(FooBar::Baz { a: 12, b: 23 }),
        _ => Err(rustler::Error::RaiseAtom("invalid_kind")),
    }
}
