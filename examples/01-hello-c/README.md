# Пример 1

Извикване на C функция от Elixir.

Дефинираме еликсирски модул в `lib/hello_c.ex`.
Дефинираме C функциите в `nif/hello.c`.

# Изпълнение

Компилирането на C кода не е добавено в `mix.exs`, затова трябва да се направи предварително:

```sh
./nif/build.sh
mix compile
```

И накрая:

```sh
mix test
```
