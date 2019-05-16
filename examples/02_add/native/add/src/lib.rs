use rustler::{rustler_export_nifs, Encoder, Env, Error, Term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
    }
}

fn add<'a>(env: Env<'a>, args: &[Term]) -> Result<Term<'a>, Error> {
    let num1 =
        match args[0].decode::<i64>() {
            Ok(num) => num,
            Err(err) => return Err(err.into()),
        };

    let num2 =
        match args[1].decode::<i64>() {
            Ok(num) => num,
            Err(err) => return Err(err.into()),
        };

    Ok((num1 + num2).encode(env))
}

fn add_v2<'a>(env: Env<'a>, args: &[Term]) -> Result<Term<'a>, Error> {
    let num1 = args[0].decode::<i64>()?;
    let num2 = args[1].decode::<i64>()?;
    Ok((num1 + num2).encode(env))
}

fn add_v3<'a>(env: Env<'a>, args: &[Term]) -> Result<Term<'a>, Error> {
    let num1 = args[0].decode::<i64>()?;
    let num2 = args[1].decode::<i64>()?;
    let result_tuple = (atoms::ok(), num1 + num2);
    Ok((result_tuple).encode(env))
}

rustler_export_nifs!(
    "Elixir.Add",
    [
        ("add", 2, add),
        ("add_v2", 2, add_v2),
        ("add_v3", 2, add_v3),
    ],
    None
);
