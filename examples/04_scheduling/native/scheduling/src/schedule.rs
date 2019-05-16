use rustler::{Env, Term, NifResult, Encoder};
use erlang_nif_sys::erlang_nif_sys_api as api;

use crate::atoms;

// Дефиницията на enif_schedule_nif е грешна в erlang_nif_sys версия 6.4.
// Грешката е поправена, но не е излязла следващата версия.
// Междувременно можем да си дефинираме правилната функция сами.
extern "C" {
    fn enif_schedule_nif(
        env: *mut api::ErlNifEnv,
        fun_name: *const u8,
        flags: i32,
        fp: unsafe extern "C" fn(env: *mut api::ErlNifEnv, argc: i32, argv: *const api::ERL_NIF_TERM) -> api::ERL_NIF_TERM,
        argc: i32,
        argv: *const api::ERL_NIF_TERM
    ) -> api::ERL_NIF_TERM;
}

// Това се извиква от Elixir
pub fn do_work<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let term = unsafe {
        let c_term = enif_schedule_nif(
            env.as_c_arg(),
            b"unsafe_do_work\0".as_ptr(),
            0,
            unsafe_do_work,
            0,
            std::ptr::null(),
        );

        Term::new(env, c_term)
    };

    // This crashes the VM
    // env.send(&env.pid(), term);

    Ok(term.encode(env))
}

// Това се извиква от nif-a do_work
unsafe extern "C" fn unsafe_do_work(env: *mut api::ErlNifEnv, _argc: i32, _argv: *const api::ERL_NIF_TERM) -> api::ERL_NIF_TERM {
    let marker = ();
    let env = Env::new(&marker, env);

    env.send(&env.pid(), atoms::work().encode(env));

    atoms::work().encode(env).as_c_arg()
}
