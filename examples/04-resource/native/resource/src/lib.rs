use rustler::{Env, ResourceArc, Term};

struct Person {
    name: String,
    age: u32,
}

#[rustler::nif]
fn make(name: &str, age: u32) -> ResourceArc<Person> {
    ResourceArc::new(Person {
        name: String::from(name),
        age,
    })
}

#[rustler::nif]
fn get_name(person: ResourceArc<Person>) -> String {
    person.name.clone()
}

#[rustler::nif]
fn get_age(person: ResourceArc<Person>) -> u32 {
    person.age
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Person, env);
    true
}

rustler::init!("Elixir.Person", [make, get_name, get_age], load = load);
