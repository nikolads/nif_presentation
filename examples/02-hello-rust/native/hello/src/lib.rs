#[rustler::nif]
fn hello() -> &'static str {
    "Здравей от Rust!"
}

rustler::init!("Elixir.HelloRust", [hello]);
