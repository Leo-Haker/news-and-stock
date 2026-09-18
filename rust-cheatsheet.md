# Rust – Cheat Sheet för nybörjare

## Cargo (projektverktyget)

```bash
cargo new namn        # nytt projekt
cargo build            # kompilera
cargo run              # kompilera + kör
cargo check            # snabb felkontroll utan att bygga binär
cargo add crate_namn   # lägg till ett beroende (nyare cargo-versioner)
```

`Cargo.toml` = projektets manifest (beroenden, metadata).
`Cargo.lock` = exakta versioner som faktiskt används — committa denna för binärer/appar.

---

## Variabler och mutabilitet

```rust
let x = 5;          // immutable (kan inte ändras)
let mut y = 5;       // mutable (kan ändras)
y = 6;
const MAX: i32 = 100; // konstant, alltid immutable, måste ha typ
```

Rust är **default immutable** — det är en medveten designval för säkerhet.

---

## Grundtyper

```rust
i32, i64, u32, u64, usize   // heltal (signed/unsigned), usize = för index/längder
f32, f64                     // flyttal
bool                          // true/false
char                          // en unicode-bokstav, t.ex. 'a'
```

## String vs &str

Det här är den vanligaste nybörjar-förvirringen:

```rust
let s1: &str = "hej";           // sträng-slice, oftast en referens till text som redan finns någonstans
let s2: String = String::from("hej"); // ägd, växande sträng på heapen
let s3: String = s1.to_string(); // konvertera &str -> String
let s4: &str = &s2;               // låna en String som &str
```

Grov regel: använd `&str` för funktionsparametrar (läsa text), `String` när du behöver äga/bygga/ändra texten.

---

## Ownership (ägarskap) — Rusts kärnkoncept

Varje värde har **en ägare**. När ägaren går ur scope, städas värdet bort automatiskt (ingen garbage collector behövs).

```rust
let s1 = String::from("hej");
let s2 = s1;       // s1 "flyttas" till s2 — s1 är inte längre giltig!
// println!("{}", s1); // FEL: s1 har flyttats

let s3 = s2.clone(); // explicit kopiera om du vill ha båda kvar
```

**Referenser (`&`) låter dig "låna" utan att ta ägarskap:**

```rust
fn skriv_ut(s: &String) {   // lånar, äger inte
    println!("{}", s);
}

let s = String::from("hej");
skriv_ut(&s);   // s är fortfarande giltig efter detta
```

Regler för referenser:
- Du kan ha **flera** `&T` (läs-referenser) samtidigt.
- Du kan ha **endast en** `&mut T` (skriv-referens) samtidigt, och inte blanda med läs-referenser.
- Detta förhindrar data races vid compile-time — inget att felsöka vid körning.

---

## Funktioner

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // sista uttrycket utan `;` = returvärde (inget `return` behövs)
}
```

---

## Structs

```rust
struct Quote {
    symbol: String,
    price: f64,
}

let q = Quote { symbol: String::from("VOLV-B.ST"), price: 280.5 };
println!("{} kostar {}", q.symbol, q.price);
```

Metoder läggs i en `impl`-block:

```rust
impl Quote {
    fn is_expensive(&self) -> bool {
        self.price > 500.0
    }
}

q.is_expensive();
```

---

## Enums + match — Rusts svar på null-säkerhet

```rust
enum Riktning {
    Upp,
    Ner,
    Annan(String),   // enum-varianter kan bära data
}

let r = Riktning::Annan(String::from("Väster"));

match r {
    Riktning::Upp => println!("upp"),
    Riktning::Ner => println!("ner"),
    Riktning::Annan(s) => println!("annan: {}", s),
}
```

### `Option<T>` — ersätter null

```rust
let maybe_number: Option<i32> = Some(5);
let nothing: Option<i32> = None;

match maybe_number {
    Some(n) => println!("fick {}", n),
    None => println!("inget värde"),
}

// Snabbare varianter:
let n = maybe_number.unwrap_or(0);       // default om None
let n2 = maybe_number.unwrap();          // panicar om None – använd sparsamt!
```

### `Result<T, E>` — ersätter exceptions

```rust
fn dela(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division med noll"))
    } else {
        Ok(a / b)
    }
}

match dela(10.0, 2.0) {
    Ok(v) => println!("resultat: {}", v),
    Err(e) => println!("fel: {}", e),
}
```

### `?`-operatorn — kortform för felpropagering

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://exempel.se")?; // om Err, returnera direkt från main
    let text = resp.text()?;
    println!("{}", text);
    Ok(())
}
```
`?` är samma som "om detta är `Err`, returnera det felet direkt från funktionen; annars fortsätt med värdet".

---

## Collections

```rust
// Vec — växande lista
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
for x in &v {
    println!("{}", x);
}

// HashMap — nyckel/värde
use std::collections::HashMap;
let mut m: HashMap<String, i32> = HashMap::new();
m.insert(String::from("ett"), 1);
if let Some(v) = m.get("ett") {
    println!("{}", v);
}
```

---

## Iteratorer och closures

```rust
let v = vec![1, 2, 3, 4, 5];

let sum: i32 = v.iter().sum();
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
```
`|x| x * 2` är en **closure** — en anonym funktion, ofta använd med iteratorer.

---

## Traits — Rusts gränssnitt/interfaces

```rust
trait Beskrivbar {
    fn beskriv(&self) -> String;
}

impl Beskrivbar for Quote {
    fn beskriv(&self) -> String {
        format!("{}: {} kr", self.symbol, self.price)
    }
}
```

---

## Felhantering i praktiken

```rust
use anyhow::Result;   // populärt crate för enklare felhantering

fn gor_nagot() -> Result<()> {
    let data = std::fs::read_to_string("fil.txt")?;
    println!("{}", data);
    Ok(())
}
```

---

## Async (för HTTP-anrop, m.m.)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://exempel.se").await?;
    let text = resp.text().await?;
    println!("{}", text);
    Ok(())
}
```
`async fn` pausar exekvering vid `.await` istället för att blockera hela tråden — bra för många samtidiga nätverksanrop. `reqwest::blocking::get` (utan async) är enklare för enkla script.

---

## Vanliga felmeddelanden och vad de betyder

| Fel | Betyder |
|---|---|
| `borrow of moved value` | Du använde ett värde efter att ägarskapet flyttats. Fixa med `.clone()` eller referenser (`&`). |
| `cannot borrow as mutable` | Du försöker ändra något som bara är lånat som läsbart (`&` istf `&mut`). |
| `mismatched types` | Fel typ skickas in, t.ex. `&str` där `String` förväntas (eller vice versa) — fixa med `.to_string()` eller `&`. |
| `the trait bound X is not satisfied` | En generisk funktion kräver att typen implementerar ett visst trait (t.ex. `Display` för `println!`). |

---

## Snabbreferens: printa saker

```rust
println!("{}", x);   # Rust – Cheat Sheet för nybörjare

## Cargo (projektverktyget)

```bash
cargo new namn        # nytt projekt
cargo build            # kompilera
cargo run              # kompilera + kör
cargo check            # snabb felkontroll utan att bygga binär
cargo add crate_namn   # lägg till ett beroende (nyare cargo-versioner)
```

`Cargo.toml` = projektets manifest (beroenden, metadata).
`Cargo.lock` = exakta versioner som faktiskt används — committa denna för binärer/appar.

---

## Variabler och mutabilitet

```rust
let x = 5;          // immutable (kan inte ändras)
let mut y = 5;       // mutable (kan ändras)
y = 6;
const MAX: i32 = 100; // konstant, alltid immutable, måste ha typ
```

Rust är **default immutable** — det är en medveten designval för säkerhet.

---

## Grundtyper

```rust
i32, i64, u32, u64, usize   // heltal (signed/unsigned), usize = för index/längder
f32, f64                     // flyttal
bool                          // true/false
char                          // en unicode-bokstav, t.ex. 'a'
```

## String vs &str

Det här är den vanligaste nybörjar-förvirringen:

```rust
let s1: &str = "hej";           // sträng-slice, oftast en referens till text som redan finns någonstans
let s2: String = String::from("hej"); // ägd, växande sträng på heapen
let s3: String = s1.to_string(); // konvertera &str -> String
let s4: &str = &s2;               // låna en String som &str
```

Grov regel: använd `&str` för funktionsparametrar (läsa text), `String` när du behöver äga/bygga/ändra texten.

---

## Ownership (ägarskap) — Rusts kärnkoncept

Varje värde har **en ägare**. När ägaren går ur scope, städas värdet bort automatiskt (ingen garbage collector behövs).

```rust
let s1 = String::from("hej");
let s2 = s1;       // s1 "flyttas" till s2 — s1 är inte längre giltig!
// println!("{}", s1); // FEL: s1 har flyttats

let s3 = s2.clone(); // explicit kopiera om du vill ha båda kvar
```

**Referenser (`&`) låter dig "låna" utan att ta ägarskap:**

```rust
fn skriv_ut(s: &String) {   // lånar, äger inte
    println!("{}", s);
}

let s = String::from("hej");
skriv_ut(&s);   // s är fortfarande giltig efter detta
```

Regler för referenser:
- Du kan ha **flera** `&T` (läs-referenser) samtidigt.
- Du kan ha **endast en** `&mut T` (skriv-referens) samtidigt, och inte blanda med läs-referenser.
- Detta förhindrar data races vid compile-time — inget att felsöka vid körning.

---

## Funktioner

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // sista uttrycket utan `;` = returvärde (inget `return` behövs)
}
```

---

## Structs

```rust
struct Quote {
    symbol: String,
    price: f64,
}

let q = Quote { symbol: String::from("VOLV-B.ST"), price: 280.5 };
println!("{} kostar {}", q.symbol, q.price);
```

Metoder läggs i en `impl`-block:

```rust
impl Quote {
    fn is_expensive(&self) -> bool {
        self.price > 500.0
    }
}

q.is_expensive();
```

---

## Enums + match — Rusts svar på null-säkerhet

```rust
enum Riktning {
    Upp,
    Ner,
    Annan(String),   // enum-varianter kan bära data
}

let r = Riktning::Annan(String::from("Väster"));

match r {
    Riktning::Upp => println!("upp"),
    Riktning::Ner => println!("ner"),
    Riktning::Annan(s) => println!("annan: {}", s),
}
```

### `Option<T>` — ersätter null

```rust
let maybe_number: Option<i32> = Some(5);
let nothing: Option<i32> = None;

match maybe_number {
    Some(n) => println!("fick {}", n),
    None => println!("inget värde"),
}

// Snabbare varianter:
let n = maybe_number.unwrap_or(0);       // default om None
let n2 = maybe_number.unwrap();          // panicar om None – använd sparsamt!
```

### `Result<T, E>` — ersätter exceptions

```rust
fn dela(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division med noll"))
    } else {
        Ok(a / b)
    }
}

match dela(10.0, 2.0) {
    Ok(v) => println!("resultat: {}", v),
    Err(e) => println!("fel: {}", e),
}
```

### `?`-operatorn — kortform för felpropagering

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://exempel.se")?; // om Err, returnera direkt från main
    let text = resp.text()?;
    println!("{}", text);
    Ok(())
}
```
`?` är samma som "om detta är `Err`, returnera det felet direkt från funktionen; annars fortsätt med värdet".

---

## Collections

```rust
// Vec — växande lista
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
for x in &v {
    println!("{}", x);
}

// HashMap — nyckel/värde
use std::collections::HashMap;
let mut m: HashMap<String, i32> = HashMap::new();
m.insert(String::from("ett"), 1);
if let Some(v) = m.get("ett") {
    println!("{}", v);
}
```

---

## Iteratorer och closures

```rust
let v = vec![1, 2, 3, 4, 5];

let sum: i32 = v.iter().sum();
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
```
`|x| x * 2` är en **closure** — en anonym funktion, ofta använd med iteratorer.

---

## Traits — Rusts gränssnitt/interfaces

```rust
trait Beskrivbar {
    fn beskriv(&self) -> String;
}

impl Beskrivbar for Quote {
    fn beskriv(&self) -> String {
        format!("{}: {} kr", self.symbol, self.price)
    }
}
```

---

## Felhantering i praktiken

```rust
use anyhow::Result;   // populärt crate för enklare felhantering

fn gor_nagot() -> Result<()> {
    let data = std::fs::read_to_string("fil.txt")?;
    println!("{}", data);
    Ok(())
}
```

---

## Async (för HTTP-anrop, m.m.)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://exempel.se").await?;
    let text = resp.text().await?;
    println!("{}", text);
    Ok(())
}
```
`async fn` pausar exekvering vid `.await` istället för att blockera hela tråden — bra för många samtidiga nätverksanrop. `reqwest::blocking::get` (utan async) är enklare för enkla script.

---

## Vanliga felmeddelanden och vad de betyder

| Fel | Betyder |
|---|---|
| `borrow of moved value` | Du använde ett värde efter att ägarskapet flyttats. Fixa med `.clone()` eller referenser (`&`). |
| `cannot borrow as mutable` | Du försöker ändra något som bara är lånat som läsbart (`&` istf `&mut`). |
| `mismatched types` | Fel typ skickas in, t.ex. `&str` där `String` förväntas (eller vice versa) — fixa med `.to_string()` eller `&`. |
| `the trait bound X is not satisfied` | En generisk funktion kräver att typen implementerar ett visst trait (t.ex. `Display` för `println!`). |

---

## Snabbreferens: printa saker

```rust
println!("{}", x);          // visa värde (kräver Display-trait)
println!("{:?}", x);        // debug-visa (kräver Debug-trait, ofta via #[derive(Debug)])
println!("{:#?}", x);       // "snygg" debug-utskrift, flera rader
```       // visa värde (kräver Display-trait)
println!("{:?}", x);        // debug-visa (kräver Debug-trait, ofta via #[derive(Debug)])
println!("{:#?}", x);       // "snygg" debug-utskrift, flera rader
```