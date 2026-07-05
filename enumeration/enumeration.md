# Les énumérations et le filtrage par motif

## Définir une énumération

Les énumérations permettent de définir des types de données personnalisées (mais différemment des structures).

### Exemple avec les adresses IP

```rust
enum TypeAdresseIP { 
    V4, 
    V6 
}
```

Nous avons créé un nouveau type appelé TypeAdresseIP avec deux valeurs possibles : V4 et V6. On peut créer une nouvelle
instance de ce type ainsi :

```rust
let typeV4 = TypeAdresseIP::V4;
```

Pour l'instant, on ne stocke pas la donnée (l'adresse IP), mais uniquement l'information concernant le type d'adresse IP.
On pourrait résoudre ce problème avec une structure :

```rust
fn main() {
    enum SorteAdresseIp {
        V4,
        V6,
    }

    struct AdresseIp {
        sorte: SorteAdresseIp,
        adresse: String,
    }

    let local = AdresseIp {
        sorte: SorteAdresseIp::V4,
        adresse: String::from("127.0.0.1"),
    };
    
    let rebouclage = AdresseIp {
        sorte: SorteAdresseIp::V6,
        adresse: String::from("::1"),
    };
}
```

Mais les énumérations en Rust permettent de faire bien mieux !

```rust
fn main() {
    enum AdresseIP {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let ip_local = AdresseIP::V4(127, 0, 0, 1);
    let ip_loop = AdresseIP::V6(String::from("::1"));
}
```

Rust propose dans sa bibliothèque standard des types de données qui permettent de manipuler les adresses IP : `IpAddr`.

```rust
struct Ipv4Addr {
    // -- code masqué ici --
}

struct Ipv6Addr {
    // -- code masqué ici --
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Cet extrait simplifié de code montre que l'on peut manipuler n'importe quel type de données dans les énumérations !

### Autre exemple d'énumération complexe

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32)
}
```

Cette énumération a quatre variantes avec des types différents :

- `Quitter` n'a aucune donnée,
- `Deplacer` intègre une structure anonyme
- `Ecrire` intègre une `String`
- `ChangerCouleur` intègre trois nombres `i32`

### L'énumération `Option`

`Option` est une énumération de la bibliothèque standard de Rust. Cela représente un cas assez courant : quand une valeur
peut-être, soit "quelque chose", soit rien du tout. Cela va avec le fait que Rust a refusé d'implémenter la valeur `null`.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Cette énumération est considérée tellement utile, qu'elle est intégrée dans l'étape préliminaire. Cela signifie qu'il 
n'est pas nécessaire de l'importer explicitement pour pouvoir l'utiliser. 

La syntaxe `<T>` est plus ou moins la même qu'en Java : il s'agit d'un type générique (à voir dans un autre chapitre). Voici 
quelques exemples :

```rust
fn main() {
    let un_nombre = Some(5);
    let une_chaine = Some("hello!");
    let un_nombre_absent: Option<i32> = None;
}
```


## La structure de contrôle de flux `match`

Voyons cela directement avec un exemple :

```rust
#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // ...
    Washington
}

enum PieceUS {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn valeur_en_centimes(piece: PieceUS) -> u8 {
    match piece {
        PieceUS::Penny => {
            println!("Un centime porte-bonheur");
            1
        },
        PieceUS::Nickel => 5,
        PieceUS::Dime => 10,
        PieceUS::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'état {:?}", etat);
            25
        },
    }
}
```

Cet exemple illustre bien les différentes fonctionnalités du `match`. On peut remarquer une chose importante : les 
`match` sont toujours **exhaustifs** !

On peut toutefois utiliser les motifs génériques ou `_` pour dire "toutes les autres valeurs de l'énumération". 

### Les motifs génériques

```rust
fn main() {
    let jet_de_de = 9;
    match jet_de_de {
        3 => println!("Met un chapeau !"),
        7 => println!("Retire un chapeau !"),
        autre => println!("Déplace toi de {} cases", autre),
    }
}
```

### Le motif `_`

Ce motif fonctionne comme un motif générique, mais permet en plus d'indiquer que l'on ne souhaite pas connaitre la valeur
réelle. En Java cela correspond au `default`.

```rust
fn main() {
    let jet_de_de = 9;
    match jet_de_de {
        3 => println!("Met un chapeau !"),
        7 => println!("Retire un chapeau !"),
        _ => (),
    } 
}
```

Avec la ligne `_ => (),`, on indique au compilateur de **ne rien faire** si la valeur du `match` n'est pas `3` ou `7` et que cela
est normal pour le programme.


## La structure de contrôle `if let`

Pour des cas simples de `match`, on peut simplifier l'écriture avec le `if let`. Prenons par exemple ce code :

```rust
fn main() {
    let une_valeur = Some(3u8);
    match une_valeur { 
        Some(max) => println!("Le maximum est réglé sur {}", max),
        _ => (),
    }
}
```

On peut simplifier cette écriture de cette façon pour un résultat équivalent :

```rust
fn main() {
    let une_valeur = Some(3u8);
    
    if let Some(max) = une_valeur {
        println!("Le maximum est réglé sur {}", max),
    }
}
```





