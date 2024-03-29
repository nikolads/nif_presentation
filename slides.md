# NIFs
## Интеграция със C и Rust код

---

# NIF

- Native Implemented Function
- интерфейс, през който BEAM зарежда и извиква C функции от динамична библиотека (.dll / .so)
- (при други езици се нарича FFI - foreign function interface)

---

# Защо

- по-бърз код, отколкото е възможно на чист Elixir
- използване на библиотека, която съществува само за C (или някой друг език)

---

# Минимален пример (C код)

```c
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
```

---

# Минимален пример (C код)

- дефинираме функции от следния тип
```
(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[]) -> ERL_NIF_TERM
```

- оказваме коя native функция на коя Erlang функция съответства.
  Например искаме C фунцкията `hello` да се достъпва в Elixir като `hello/0`
```
static ErlNifFunc nif_funcs[] = {
    {"hello", 0, hello}
};
```

- извикваме макрото `ERL_NIF_INIT`, което генерира допълнителна информация, нужна на BEAM за да зареди библиотеката. Това включва името на Erlang/Elixir модула.

---

# Минимален пример (Elixir код)

```elixir
defmodule HelloC do
  @on_load :load_nifs
  def load_nifs do
    :ok = :erlang.load_nif("./nif/_build/libhello", 0)
  end

  def hello, do: :erlang.nif_error(:nif_not_loaded)
end
```

---

# Минимален пример (Elixir код)

- дефинираме модул, който ще държи nif-овете. Nif-овете винаги се асоциират с модул. Библиотеката ще стои заредена докато съответния модул е зареден.

- дефинираме самите функции с имплементация по подразбиране. При успешно зареждане на библиотеката те ще се заместят със съответната native фунцкция.

- извикваме `:erlang.load_nif(<път до библиотеката>, 0)`, което връща `:ok` ако библиотеката се зареди успешно

---

# Директива @on_load

Може да се използва `@on_load: <function>` за да се зареди библиотеката при зареждане на модула

- `@on_load` създава нов процес в който изпълнява подадената функция
- процеса умира когато функцията завърши
- при успешно зарешдане функцията трябва да върне `:ok`
- ако друг процес се опита да извика функция от този модул, той ще бъде спрян докато `@on_load` функцията не завърши (това работи само при първоначалното зареждане, но не при code change)

---

# Документация

https://www.erlang.org/doc/man/erl_nif.html

---

# Ограничения

- NIF-овете ефективно разширяват кода на виртуалната машина
- изпълняват се в незащитена среда, затова трябва да сме изключително внимателни, когато ги използваме
- ако NIF "гръмне" - ще убие цялата виртуална машина
- ако NIF предизвика memory corruption - много лошо
- ако NIF се изпълнява твърде дълго време - може да възпрепядства scheduler-ите на BEAM

---

# Rustler

- библиотека на Rust за писане на erlang-ски NIF-ове
- елиминира голяма част от boilerplate кода за интеграция с BEAM
- фокус върху безопасност - няма начин да се предизвика краш в BEAM
- https://docs.rs/rustler/
- https://hexdocs.pm/rustler/

---

# Rust

- език от ниско ниво - директен контрол над генерирания код, над паметта, над алокациите
- език от високо ниво - декларативен стил на писане, силна типова система, union типове, pattern matching
- модерен език с модерен tool-инг
- напълно безопасен език - предизвикването на UB е невъзможно

---

# Rustler

Добавяме elixir-ската част от библиотеката като dependency
```elixir
defp deps do
  [
    {:rustler, "~> 0.28.0"},
  ]
end
```

Това включва генератор, който ще ни създаде rust-ска библиотека в `native/<име>/`

```
mix deps.get
mix rustler.new
```

---

# Rustler пример

## Rust код

```rust
#[rustler::nif]
fn hello() -> &'static str {
    "Здравей от Rust!"
}

rustler::init!("Elixir.HelloRust", [hello]);
```

---

# Rustler пример

## Elixir код

```elixir
defmodule HelloRust do
  use Rustler, otp_app: :hello_rust, crate: :hello

  def hello, do: :erlang.nif_error(:nif_not_loaded)
end
```

Редът `use Rustler, ...` навързва двата проекта, при което при компилиране на elixir кода автоматично ще се компилира и rust кода

---

# Работа с термове

В най-простия си вариант, един NIF в Rusltler получава средата `Env` и списък от термове `Term` и връща `Term`.
`#[rustler::nif]` е макрос, който генерира нужнуя код за извикването на тази функция от C API-то.

`Env` е необходим за създаване на нови термове, както и повечето неща които wrap-ват функции от C API-то. Този аргумент може да се пропусне, ако не е неоходим.

`Term` в Rustler съдържа не само `ERL_NIF_TERM`, но и референция към `Env`, което позволява да му се викат методи без да се подава `env` навсякъде.

```rust
#[rustler::nif]
fn my_func<'a>(env: Env<'a>, arg1: Term<'a>, arg2: Term<'a>) -> Term<'a> { ... }
```

---

# Работа с термове

NIF-а също може директно да приема и връща типове, които могат да се конвертират от и до `Term`.
В такъв случай библиотеката се грижи за конвертирането.
Ако подадените аргументи се различават от очаквания тип се хвърля erlang-ска грешка

```rust
fn my_func(arg1: Atom, arg2: i32) -> String { ... }
```

---

# Работа с термове

Rustler също поддържа derive макроси, които генерират конвертиране от rust-ска структура до еликсирски map/struct/exception. Също така от rust-ски enum-и до еликсирски tuple-и.

Поддържа се и автоматично конвертиране от стандартните rust-ски типове `Option` и `Result`
```
Option
- Some(val) -> val
- None      -> :nil

Result
- Ok(val)   -> {:ok, val}
- Err(e)    -> {:error, e}
```

---

# Работа с термове

Атомите могат да се дефинират предварително, за да не се налага да се конструират от низ всеки път

```rust
rustler::atoms! {
    ok,
    error,
    unknown_term,
    foo,
    bar,
    baz,
}
```

---

# Запазване на термове

- `Term`-овете, подадени като аргументи на NIF функция са валидни само по времето на тази функция
- след края на функцията GC може да ги изтрие по всяко време
- (в Rustler това е оказано чрез lifetime анотации и опита да се задържи `Term` за по-дълго от текущата функция би довел до компилационна грешка)

---

# Запазване на термове

За да се запази терм за по-дълго може:
- да се създаде `OwnedEnv` - отделен heap, който не е асоцииран с процес и да се копира терма там
- да се сериализира до [External Text Format](http://erlang.org/doc/apps/erts/erl_ext_dist.html) на Erlang. Това връща `OwnedBinary`, което може по-късно да се десериализира до `Term` в някой `Env`.

---

# Ресурси

- ресурсите позволяват NIF-овете да работят със собствени типове, а не само с erlang-ски термове
- трябва да се регистрира типа на ресурса, след това могат да се създават обекти от този тип
- за всеки обект виртуалната машина заделя памет и връща handle към тази памет
- този handle може да бъде превърнат в erlang-ски терм и върнат от NIF-а
- термът е opaque от гледна точка на Erlang/Elixir
- може да бъде запазен и препращан, но не и използван директно
- може да бъде подаден като аргумент на NIF, от където може да се достъпи оригиналната структура

---

# Ресурси

- типа на ресурса трябва да бъде регистриран по време на зареждане на библиотеката
- възможно е да подадем функции, които ще се извикат при определено събитие, свързано с nif библиотеката
- събитията са `load`, `upgrade`, `unload`
- тип на ресурс се дефинира по време на `load` или `upgrade`

---

# Ресурси

В C API-то функциите се подават на макрото `ERL_NIF_INIT`

```c
ERL_NIF_INIT(MODULE, nif_funcs, load, NULL, upgrade, unload)
```

Rustler за момента поддържа само `load` функцията

```rust
rustler::init!("MODULE", nif_funcs, load = load);
```

---

# Ресурси

- паметта за ресурс обектите се контролира от Erlang
- паметта се алокира от Erlang при създаване на обекта
- всеки обект съдържа reference counter. Когато броят референции (от Erlang и от NIF библиотеката) стигне нула се извиква деструктор, зададен за съответния тип, и се освобождава паметта.

---

# Мутация

Чрез ресурси можем да имплементираме mutable състояние в Elixir.

В C това просто би работило, но не и в Rust.
Rust има правило, че една стойност не може да е едновременно споделена и mutable.

`ResourceArc<T>` ни позволява да вземем само константна референция `&T` към вътрешността, защото ресурса може да бъде копиран и споделян между множество elixir-ски процеси.

---

# Мутация

За да можем да модифицираме ресурса трябва да вземем `&mut T`, но за целта трябва да докажем, че имаме ексклузивен достъп до стойността.

1. Трябва да се подсигурим, че ресурса се достъпва само от един процес от Elixir. За целта можем да използваме `GenServer`

2. Трябва да покажем на Rust, че имаме ексклузивен достъп. За целта можем да използваме `Mutex` или `SpinLock`, но е важно никога да не блокираме, опитвайки се да заключим мутекса. Т.е. използваме само `Mutex::try_lock`, но не и `Mutex::lock`

---

# Scheduling

- NIF-овете трябва да са сравнително кратки, за да не блокират BEAM
- документацията препоръчва да не се надхвърля 1 милисекунда
- при нужда от по-дълго време за изпълнение има няколко варианта

---

# Scheduling

## schedule_nif

- работата се разделя на малки парчета
- използва се `enif_schedule_nif`, за да се schedule-не извикването на функция
- тази функция изпълнява едно парче работа и извиква `enif_schedule_nif` отново, докато цялата работа не е свършена

Това не се поддържа от Rustler все още.
Нещо подобно може да се имплементира, ако разбиването на задачата се имплементира на ниво Elixir.

---

# Scheduling

## нишки

- native библиотека пуска отделна нишка на ОС, която изпълнява задачата
- (и може би поддържа thread pool от такива нишки)
- NIF-а връща веднага
- истинския резултат се изпраща като съобщение с `env.send`

---

# Scheduling

## dirty nif

- задава се с флаг, че въпросния NIF е "мръсен"
- DirtyIO или DirtyCPU, в зависимост дали операцията блокира заради IO или е тежка от към процесорен ресурс
- BEAM изпълнява такива "мръсни" NIF-ове в отделен thread pool с отделни scheduler-и

При Rustler това се оказва чрез аргумент към `rustler::nif` макрото

```rust
#[rustler::nif(schedule = "DirtyCpu")]
pub fn my_lengthy_work() -> i64 {
    let duration = Duration::from_millis(100);
    std::thread::sleep(duration);
    42
}
```

---

# Материали

Код от примерите
https://github.com/nikolads/nif_presentation/tree/2023/examples

---

# Въпроси?
