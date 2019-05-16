## Интеграция с други езици
### (Nifs)

---

## Nif

@ul
- Native Implemented Functions
- функции дефинирани на произволен език, компилирани до динамична библиотека (.dll / .so)
- виртуалната машина зарежда библиотеката и позволява използването им все едно са нормални Erlang функции
@ulend

---

## Nif

@ul
- nif-овете ефектвно разширяват кода на виртуалната машина
- изпълняват се в незащитена среда, в същата ОС нишка която изпълнява Erlang код
- трябва да сме изключително внимателни когато ги използваме
  - ако nif "гръмне", той ще убие цялата виртуална машина
  - ако nif се изпълнява дълго време той ще възпрепядства работата на scheduler-ите на виртуалната машина
  - виж забележката на http://erlang.org/doc/man/erl_nif.html#description

---

## Минимален пример (C код)

```c
/* niftest.c */
#include <erl_nif.h>

static ERL_NIF_TERM c_hello(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[])
{
    return enif_make_string(env, "Hello world!", ERL_NIF_LATIN1);
}

static ErlNifFunc nif_funcs[] = {
    {"hello", 0, c_hello}
};

ERL_NIF_INIT(Elixir.Niftest, nif_funcs, NULL, NULL, NULL, NULL)
```

Команда за компилиране до `niftest.so`:
`gcc -fPIC -shared -I $ERL_ROOT/usr/include/ niftest.c -o niftest.so`

---

## Минимален пример (C код)

- дефинираме функции от следния тип (`ERL_NIF_TERM` е typedef за число)
```
(env: ErlNifEnv*, argc: int, argv: ERL_NIF_TERM[]) -> ERL_NIF_TERM
```

- оказваме коя native функция на коя Erlang функция съответства.
  Например искаме C фунцкията `c_hello` да се достъпва като `hello/0`
```
{"hello", 0, c_hello}
```

- извикваме макрото `ERL_NIF_INIT`, което генерира допълнителна информация, нужна на BEAM за да зареди библиотеката. Това включва името на Erlang/Elixir модула.

---

## Минимален пример (Elixir код)

```elixir
defmodule Niftest do
  def load_nifs do
    :erlang.load_nif("./niftest", 0)
  end

  def hello do
    :erlang.nif_error(:nif_not_loaded)
  end
end
```

---

## Минимален пример (Elixir код)

- дефинираме модул, който ще държи nif-овете. Nif-овете винаги се асоциират с модул. Библиотеката ще стои заредена докато съответния модул е зареден.

- дефинираме самите функции с имплементация по подразбиране. При успешно зареждане на библиотеката те ще се заместят със съответната native фунцкция.

- извикваме `:erlang.load_nif(<път до библиотеката>, 0)`, което връща `:ok` ако библиотеката се зареди успешно

---

## Директива @on_load

Може да се използва `@on_load: <function>` за да се зареди библиотеката при зареждане на модула

```elixir
defmodule Niftest do
  @on_load: :load_nifs

  def load_nifs do
    :erlang.load_nif("./niftest", 0)
  end

  def hello do
    :erlang.nif_error(:nif_not_loaded)
  end
end
```

---

## Директива @on_load

@ul
- @on_load създава нов процес в който изпълнява подадената функция
- процеса умира когато функцията завърши
- при успешно зарешдане функцията трябва да върне `:ok`
- ако друг процес се опита да извика функция от този модул, той ще бъде спрян докато `@on_load` функцията не завърши (това работи само при първоначалното зареждане, но не при code change)
@ulend

---

## Rustler

@ul
- целта на тази презентация е да покаже как работят nif-овете, а не да ви научи на C API-то
- затова ще си позволя да използвам библиотека, която да намали boilerplate кода
- rustler е проект, който цели да направи комуникацията между Rust и Elixir лесна и безопазна
- съдържа библиотека на Rust (https://docs.rs/rustler) и на Elixir (https://hexdocs.pm/rustler/)
- забележка - проекта е все още в етап на разработка
@ulend

---

## Rustler

- има примери в github repo-то
- Elixir кодът е в `lib/`
- Rust кодът е в `native/<crate_name>/src/`

---

## Началният пример с rustler

```rust
use rustler::{rustler_export_nifs, Encoder, Env, NifResult, Term};

fn rust_hello<'a>(env: Env<'a>, _args: &[Term]) -> NifResult<Term<'a>> {
    Ok("Hello, world!".encode(env))
}

rustler_export_nifs!(
    "Elixir.Niftest",
    [
        ("hello", 0, rust_hello),
    ],
    None
);
```

---

## Работа с термове

```rust
fn add<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
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
```

---

## Работа с термове

@ul
- аргументите получаваме като масив от Term-ове
- трябва да ги разкодираме до съответния тип
- резултатът трябва да се кодира до Term
- функцията също така може да хвърли грешка
@ulend

---

## Работа с термове

```rust
fn add<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let num1 = args[0].decode::<i64>()?;
    let num2 = args[1].decode::<i64>()?;
    Ok((num1 + num2).encode(env))
}
```

Има и по-кратък вариант :)

---

## Работа с атоми

```rust
mod atoms {
    rustler::rustler_atoms! {
        atom ok;
    }
}

fn add<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let num1 = args[0].decode::<i64>()?;
    let num2 = args[1].decode::<i64>()?;
    let result_tuple = (atoms::ok(), num1 + num2);
    Ok(result_tuple.encode(env))
}
```

Rustler използват макро за дефиниране на атоми, но имплментацията се свежда до извикване на
`ERL_NIF_TERM enif_make_atom(ErlNifEnv* env, const char* name)`

---

## Собствени типове

Една причина поради която бихме искали да използваме nif-ове е за да използваме структури от данни, които не можем да си напишем на Elixir/Еrlang.

Ще имплементираме вектор от Erlang термове

---

## Собствени типове

@ul
- можем да приемаме и връщаме собствени структури от nif-ове. Те се наричат ресурси.
- първо трябва да дефинираме тип на ресурса.
- след това можем да създаваме обекти от този тип. За всеки обект виртуалната машина заделя памет и връща handle.
- handle-ът е обикновен Erlang терм. Erlang не може да го използва директно, но може да го подаде обратно на nif.
- обектите имат reference counter. Когато броят референции (от Erlang и от nif библиотеката) стигне нула се извиква деструктор, зададен за съответния тип, и се освобождава паметта.
@ulend

---

## Типове на ресурси

@ul
- можем да подадем функции, които ще се извикат при определено събитие свързано с nif библиотеката
- събитията са
  - `load`
  - `upgrade`
  - `unload`
- тип на ресурса може да се дефинира само от функциите `load` и `upgrade`
@ulend

---

## Типове на ресурси

@ul
- в C функциите се подават на `ERL_NIF_INIT` макрото
- в първоначалния пример бяха `NULL`
- но могат да са и `ERL_NIF_INIT(MODULE, funcs, load, NULL, upgrade, unload)`
@ulend

---

## Типове на ресурси

Ruslter за момента позволява да зададем само `load` фунцкията

```rust
struct MyVector {
    inner: Vec<?>,
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(MyVector, env);
    true
}

rustler_export_nifs! {
    "Elixir.VectorNif",
    [],
    Some(on_load)
}
```

---

## Ресурс обекти

```rust
fn vector_new<'a>(env: Env<'a>, _args: &[Term]) -> NifResult<Term<'a>> {
    let vec = ResourceArc::new(MyVector { inner: Vec::new() });
    Ok(vec.encode(env))
}

fn vector_len<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;
    let len = vec.inner.len();
    Ok(len.encode(env))
}
```

---

## Запазване на термове

@ul
- термовете живеят в процеса, който е извикал nif-a
- не можем просто да ги копираме във вектора, защото следващият път когато се опитаме да ги достъпим може вече да не са валидни
- можем да използваме [External Text Format](http://erlang.org/doc/apps/erts/erl_ext_dist.html) на Erlang
@ulend

---

## Запазване на термове

```rust
fn push<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;
    let term = args[1];
    let encoded = term.to_binary();

    vec.inner.push(encoded);
    Ok(atoms::ok().encode(env))
}

fn get<'a>(env: Env<'a>, args: &[Term]) -> NifResult<Term<'a>> {
    let vec = args[0].decode::<ResourceArc<MyVector>>()?;
    let index = args[1].decode::<usize>()?;

    match vec.inner.get(index) {
        Some(encoded) => {
            let decoded = env.binary_to_term(encoded.as_slice()).unwrap().0;
            Ok((atoms::ok(), decoded).encode(env))
        },
        None => Ok((atoms::error(), atoms::out_of_bounds()).encode(env)),
    }
}
```

---

## Мутация

@ul
- кодът от предишния слайд не се компилира
- ресурсът е споделен обект - процесът който го е създал може да изпрати handle до друг процес
- затова rust не позволява да променяме вектора
- това е проблем, защото `vector_push` трябва да промени вектора
- ако искаме да го променим трябва да покажем, че в този момент никой друг няма достъп до вектора или неговото съдържание
@ulend

---

## Мутация

@ul
- можем да използваме `Mutex` или други примитиви за заключване, но това обикновенно е лоша идея
- мутекса може да остане заключен за дълго време, през което време сме блокирали тази нишка от виртуалната машина
- по-добре е да се избегне нуждата за заключване
- примерно - можем да избегнем споделянето като сложим вектора в `Agent`. Така еликсирския процес гарантира, че вектора не може да се достъпва едновременно.
- друг вариант е да се използват специални конкурентни структури от данни. Така можем да избегнем bottleneck-а от използването на един процес.
@ulend

---

## Извикване на дълги nif-ове

@ul
- добре е времето за изпълнение на nif да не надхвърля 1 милисекунда
- ако искаме да изпълним някоя дълга операция в nif, има няколко опции
@ulend

---

## Нишки

```rust
fn echo_from_thread<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let self_pid = env.pid();
    let term = args[0];

    let thread_env = OwnedEnv::new();
    let saved_term = thread_env.save(term);

    thread::spawn(move || {
        thread_env.run(|env| {
            let term = saved_term.load(env);

            // do work

            env.send(&self_pid, term);
        });
    });

    Ok(atoms::nil().encode(env))
}
```

---

## Нишки

@ul
- можем да създаваме нишки на операционната система в които да изпълняваме дългата операция
- когато резултатът е готов можем да го изпратим
- ако трябва да изчакаме докато резултатът пристигне - използваме `receive`
@ulend

---

## Мръсни нифове

```rust
rustler_export_nifs! {
    "Elixir.Scheduling",
    [
        ("dirty_nif", 0, dirty_nif, SchedulerFlags::DirtyIo),
    ],
    None
}

fn dirty_nif<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    thread::sleep(Duration::from_secs(5));
    Ok(atoms::work().encode(env))
}
```

---

## Мръсни нифове

@ul
- сравнително нова опция на BEAM
- BEAM създава определен брой нишки, които се използват само за извикване на nif-ове
- "мръсните" нифове се декларират с определен флаг (в случая `SchedulerFlags::DirtyIo`)
- има вариант за нифове, които правят сложни операции (`DirtyCpu`) или чакат за I/O (`DirtyIo`)
@ulend

---

## Schedule nif

- не се поддържат от rustler (все още)
- ако много искате вижте примера в `examples/04_scheduling`, който извиква C функцията директно
- (примерът може да е грешен)

---

## Schedule nif

```c
enif_schedule_nif(env, "function_name", flags, function, argc, argv)
```

@ul
- функция, която позволява от nif да определим точно един нов nif, който ще се изпълни
- първоначалният nif излиза, при което връща контрол на BEAM
- новият nif продължава да се изпълнява. Той може да извика `enif_schedule_nif` отново
- резултатът от последния nif се връща на Elixir
@ulend

---

## Schedule nif

- може да се използва за разбиване на дълга операция на множество малки
- между всяка малка операция се връща контрол на BEAM
- подобен подход се среща често когато се използва кооперативна многозадачност, например `setTimeout(0)` в javascript

---

## Schedule nif

- може да се използва и за да се пусне нормален nif върху dirty scheduler
- това става като се зададе съответния флаг в параметъра `flags`
