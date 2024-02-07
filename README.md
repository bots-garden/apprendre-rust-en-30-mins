# apprendre-rust-en-30-mins

Rust est un langage comme un autre avec ses spécificités. Mais beaucoup s'en font tout un monde (une sorte d'élitisme inutile s'est installé autour de ce langage). J'espère que ce projet vous aidera à franchir le pas, mais n'oubliez pas de lire la documentation officielle (https://doc.rust-lang.org/book/), lire des livres, blog posts et demander aux copains, ...

**[🍊 Open with Gitpod](https://gitpod.io/#https://github.com/bots-garden/apprendre-rust-en-30-mins)**

### Étape 0: Installation de Rust

Vous êtes dans un projet "Gitpodifié", donc vous n'avez rien à installer. Sinon, l'installation de Rust est plutôt triviale, je vous laisse en juger: https://doc.rust-lang.org/book/ch01-01-installation.html

### Étape 1: Hello, World!

Créez un nouveau projet Rust en utilisant **Cargo** (le gestionnaire de paquets et de projets de Rust). Dans votre terminal, exécutez :

```bash
cargo new 01-hello-world --name hello_world
cd 01-hello-world
```

Ouvrez le fichier `src/main.rs` et remplacez le contenu par :

```rust
fn main() {
    println!("👋 Hello, World 🌍!");
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 2: Variables et Types de Données

Créez un nouveau projet:

```bash
cargo new 02-variables --name variables
cd 02-variables
```

Rust est un langage statiquement typé, donc vous devez déclarer le type de vos variables. Voici un exemple :

```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 3.14;
    let is_rust_cool: bool = true;

    println!("x = {}, y = {}, Rust is cool = {}", x, y, is_rust_cool);
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 3: Structures de Contrôle

Créez un nouveau projet:

```bash
cargo new 03-control-structure --name control_structure
cd 03-control-structure
```

Utilisez `if`, `else if`, et `else` pour les conditions, et `loop`, `while`, ou `for` pour les boucles :

```rust
fn main() {
    let number = 7;

    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    for i in 0..5 {
        println!("Iteration: {}", i);
    }
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 4: Fonctions

Créez un nouveau projet:

```bash
cargo new 04-functions --name functions
cd 04-functions
```

Déclarez des fonctions avec `fn` :

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let sum = add(3, 4);
    println!("Sum: {}", sum);
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 5: Ownership (posseder) et Borrowing (emprunter)

Créez un nouveau projet:

```bash
cargo new 05-ownership-borrowing --name ownership_borrowing
cd 05-ownership-borrowing 
```

Rust utilise un système de possession unique pour gérer la gestion de la mémoire. Voici un exemple simple :

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // ownership transférée à s2
    // println!("{}", s1); // erreur car s1 ne possède plus la chaîne.
    println!("{}", s2);

    let s3 = String::from("Hello");
    let s4 = &s3; // borrowing de l'ownership
    println!("{}", s3); // Cela est autorisé grâce au borrowing.
    println!("{}", s4);
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 6: Structs et Enums
> et aussi un peu de programmation fonctionnelle

Créez un nouveau projet:

```bash
cargo new 06-structs-enums --name structs_enums
cd 06-structs-enums 
```

Définissez des structures de données avec `struct` et créez des énumérations avec `enum` :

```rust
struct Person {
    name: String,
    age: u32,
}

enum Status {
    Success,
    Failure(String),
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    match person.age {
        0..=17 => println!("Minor"),
        18..=64 => println!("Adult"),
        _ => println!("Senior"),
    }

    let result = Status::Success;
    match result {
        Status::Success => println!("Operation succeeded"),
        Status::Failure(msg) => println!("Operation failed with message: {}", msg),
    }
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

✋ **Vous allez avoir quelques warnings, mais pas d'inquétude, ça va quand même s'exécuter**:

> Celui-ci, c'est parce que vous affectez une valeur à `name`, mais vous ne la lisez pas
```bash
warning: field `name` is never read
  --> src/main.rs:23:5
   |
22 | struct Person {
   |        ------ field in this struct
23 |     name: String,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default
```

> Le compilateur estime que votre `enum` n'est pas "reachable" par le reste du code. Si vous passez votre `enum` en publique (`pub`), le warning disparaîtra.
```bash
warning: variant `Failure` is never constructed
  --> src/main.rs:29:5
   |
27 | enum Status {
   |      ------ variant in this enum
28 |     Success,
29 |     Failure(String),
   |     ^^^^^^^
```

### Étape 7: Gestion des Erreurs

Créez un nouveau projet:

```bash
cargo new 07-errors --name errors
cd 07-errors 
```

Utilisez `Result` pour gérer les erreurs :

```rust
fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(msg) => println!("Error: {}", msg),
    }
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

✋ Alors, lorsque l'on vient du JavaScript comme moi, il y a un truc un peu **"ésotérique"** dans la notation du type de retour de la fonction: `Result<f64, &'static str>` (et là tu te dis que les gens qui font du Rust font ça pour pas que l'on fasse du Rust 🤭, je reconnais avoir eu la même impression quand je me suis mis à Scala).

`&'static str` est une référence à une chaîne de caractères statique en cas d'erreur 🤔 ... 🤪

Allez, je re-essaie:

La notion de `'static` en Rust se réfère à la durée de vie de la chaîne de caractères et à la manière dont cette chaîne est stockée en mémoire.

Une référence avec une durée de vie `'static` signifie qu'elle pointe vers une **région mémoire statique**, dont la durée de vie est la totalité de l'exécution du programme. Concrètement, cela signifie que la chaîne de caractères à laquelle elle fait référence est stockée dans une région mémoire **qui persiste pendant toute la durée de vie (d'exécution) du programme**.

### Étape 8: Traitement des Strings et des Vectors

Créez un nouveau projet:

```bash
cargo new 08-strings-vectors --name strings_vectors
cd 08-strings-vectors 
```

Rust propose des types de données intégrés tels que `String` et `Vec` :

```rust
fn main() {
    // String
    let mut greeting = String::from("Hello, ");
    greeting.push_str("world!");
    println!("{}", greeting);

    // Vec
    let numbers = vec![1, 2, 3, 4, 5];
    for num in &numbers {
        println!("{}", num);
    }
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

### Étape 9: Concurrency avec Threads

Créez un nouveau projet:

```bash
cargo new 09-threads --name threads
cd 09-threads 
```

Rust prend en charge le multithreading de manière plutôt simple et intuitive (mais attention, l'exemple est simple):

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Créer un vecteur pour stocker les threads
    let mut handles = vec![];

    for i in 0..5 {
        // Créer un nouveau thread pour chaque itération
        let handle = thread::spawn(move || {
            // Code exécuté dans chaque thread
            for j in 0..3 {
                println!("Thread {}: Iteration {}", i, j);
                thread::sleep(Duration::from_millis(100)); // Simuler une tâche prenant du temps
            }
        });

        // Ajouter le thread au vecteur
        handles.push(handle);
    }

    // Attendre que tous les threads se terminent
    for handle in handles {
        handle.join().unwrap();
    }
}
```

Ensuite, exécutez votre programme avec la commande :

```bash
cargo run
```

Dans cet exemple, j'ai créé cinq threads, chacun exécutant une boucle. Les threads sont stockés dans un vecteur handles. La fonction `thread::sleep` simule une tâche qui prend du temps.

Ensuite, dans la deuxième boucle on attend que tous les threads se terminent à l'aide de la méthode join. Cela permet aux threads de s'exécuter de manière concurrente, affichant leurs sorties (`println!("Thread {}: Iteration {}", i, j);`) pendant que le programme principal attend qu'ils se terminent.

✋ vous avez du remarquer le mot clé `mut` au moment de la déclaration du vecteur `handles`. Le mot-clé `mut` est utilisé pour indiquer que la variable est mutable, c'est-à-dire qu'elle peut être modifiée après sa création. Cela s'applique principalement aux variables, mais peut également être utilisé pour rendre des références mutables, des champs de structures mutables, etc ...

### Conclusion

Ceci était une introduction (très) rapide à Rust. Il vous reste encore du chemin, mais vous avez de quoi commencer sans stress.

Je vous recommande aussi ces guides d'introduction, complètement complémentaires au mien:

- [Learn Rust in under 10 mins](https://blog.ediri.io/learn-rust-in-under-10-mins)
- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
