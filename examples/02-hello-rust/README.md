# Пример 2

Извикване на Rust функция от Elixir.

Дефинираме еликсирски модул в `lib/hello_rust.ex`.
Дефинираме Rust функциите в `native/hello/src/lib.rs`.

# Генериране на проект

Rustler съдържа `mix` команда, която може директно да ни генерира rust-ския подпроект.

```
mix new hello_rust
# add {:rustler, "~> 0.28.0"} to deps in mix.exs
mix deps.get
mix compile
mix rustler.new
```

# Изпълнение

Редът `use Rustler, otp_app: :hello_rust, crate: :hello` в `hello_rust.ex` навързва двата проекта, така че rust кода ще се компилира автоматично, когато се компилира elixir кода`

```
mix test
```
