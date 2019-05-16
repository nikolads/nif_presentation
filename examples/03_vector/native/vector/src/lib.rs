use rustler::types::OwnedBinary;
use rustler::{
    resource_struct_init, rustler_export_nifs, Encoder, Env, NifResult, ResourceArc, Term,
};
use std::sync::{Mutex, TryLockError};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom out_of_bounds;
    }
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(MyVector, env);
    true
}

rustler_export_nifs! {
    "Elixir.VectorNif",
    [
        ("new", 0, vector_new),
        ("len", 1, vector_len),
        ("push", 2, vector_push),
        ("get", 2, vector_get),
    ],
    Some(on_load)
}

struct MyVector {
    inner: Mutex<Vec<OwnedBinary>>,
}

fn vector_new<'a>(env: Env<'a>, _args: &[Term]) -> NifResult<Term<'a>> {
    let vec = ResourceArc::new(MyVector {
        inner: Mutex::new(Vec::new()),
    });
    Ok(vec.encode(env))
}

fn vector_len<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;

    let result =
        match vec.inner.try_lock() {
            Ok(lock) => Ok(lock.len().encode(env)),
            Err(TryLockError::WouldBlock) => Err(rustler::Error::BadArg), // TODO: better error
            _ => panic!("should not happen"),
        };

    result
}

fn vector_push<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;
    let term = args[1];
    let encoded = term.to_binary();

    let result =
        match vec.inner.try_lock() {
            Ok(mut lock) => {
                lock.push(encoded);
                Ok(atoms::ok().encode(env))
            },
            Err(TryLockError::WouldBlock) => Err(rustler::Error::BadArg), // TODO: better error
            _ => panic!("should not happen"),
        };

    result
}

fn vector_get<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;
    let index = args[1].decode::<usize>()?;

    let result =
        match vec.inner.try_lock() {
            Ok(lock) => match lock.get(index) {
                Some(encoded) => {
                    let decoded = env.binary_to_term(encoded.as_slice()).unwrap().0;
                    Ok((atoms::ok(), decoded).encode(env))
                },
                None => Ok((atoms::error(), atoms::out_of_bounds()).encode(env)),
            },
            Err(TryLockError::WouldBlock) => Err(rustler::Error::BadArg), // TODO: better error
            _ => panic!("should not happen"),
        };

    result
}
