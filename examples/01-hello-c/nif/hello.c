#include <erl_nif.h>
#include <string.h>

static ERL_NIF_TERM hello(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[]) {
    const char* greeting = "Здравей от C!";
    const size_t greeting_len = strlen(greeting);

    ERL_NIF_TERM new_binary;
    unsigned char* new_binary_data = enif_make_new_binary(env, greeting_len, &new_binary);
    memcpy(new_binary_data, greeting, greeting_len);

    return new_binary;
}

static ErlNifFunc nif_funcs[] = {
    {"hello", 0, hello}
};

ERL_NIF_INIT(Elixir.HelloC, nif_funcs, NULL, NULL, NULL, NULL)
