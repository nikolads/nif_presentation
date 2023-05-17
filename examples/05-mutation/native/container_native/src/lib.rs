use rustler::{Atom, Env, ResourceArc, Term};
use std::collections::hash_map::{HashMap, Entry};
use std::sync::Mutex;

mod atom {
    rustler::atoms! {
        inserted,
        occupied,
    }
}

struct Container {
    data: Mutex<HashMap<String, String>>,
}

#[rustler::nif]
fn new() -> ResourceArc<Container> {
    ResourceArc::new(Container {
        data: Mutex::new(HashMap::new()),
    })
}

#[rustler::nif]
fn insert(container: ResourceArc<Container>, key: String, val: String) -> Atom {
    let Ok(mut lock) = container.data.try_lock() else {
        panic!("container instance should not be shared")
    };

    let data = &mut *lock;
    match data.entry(key) {
        Entry::Vacant(vacant) => {
            vacant.insert(val);
            atom::inserted()
        }
        Entry::Occupied(_) => atom::occupied(),
    }
}

#[rustler::nif]
fn get(container: ResourceArc<Container>, key: String) -> Option<String> {
    let Ok(lock) = container.data.try_lock() else {
        panic!("container instance should not be shared")
    };

    let data = &*lock;
    data.get(&key).cloned()
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Container, env);
    true
}

rustler::init!("Elixir.Container.Native", [new, insert, get], load = load);
