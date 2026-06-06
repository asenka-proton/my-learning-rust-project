# Le type Slice

Une _slice_ permet d'obtenir une référence vers une séquence continue d'éléments d'une collection plutôt que toute la 
collection.

## Un exemple de problème

Écrire une fonction qui prend une chaine de caractères en retourne le premier mot dans cette chaine.

```rust
fn premier_mot(s: &String) -> &str {
    
    const SPACE = b' ';
    let octets = s.as_bytes();
    
    for (i, &element) in octets.iter().enumerate() {
        if element == SPACE {
            return &s[0..i];
        }
    }
    &s[..]
}
```

Afin de pouvoir parcourir la `String` élément par élément et vérifier si la valeur est une espace, il faut convertir 
`s` en un tableau d'octets avec `s.as_bytes()`. Ensuite, on créé un itérateur sur ce tableau avec `iter()`. 

> Note:
> 
> - la fonction `iter()` retourne chaque élément d'une collection
> - la fonction `enumerate()` transforme chaque élément en un **tuple** : 
>   - le premier élément est l'indice
>   - le deuxième élément est une référence vers l'élément

Si on tombe sur un espace, on va retourner une référence vers une slice qui prend les caractères de 0 à i (l'indice 
où l'espace a été identifié) : `&[0..i]`.

Désormais, quand nous appelons `premier_mot`, nous récupérons une unique valeur qui est liée à la donnée de base. 
La valeur se compose d'une référence vers le point de départ de la slice et du nombre d'éléments dans la slice.

## Les slices de caractères

Une slice de chaine de caractères (ou _slice de chaine_) est une référence à une partie d'une `String`. Par exemple:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

Schématiquement, en mémoire, cela donne ceci (pour `s` et `world`):

```
          s                            
   ┌───────────┬──────┐              ┌──────┬────────┐
   │  ptr      │  ────┼───────────►  │  0   │  'H'   │
   │  taille   │  11  │              │  1   │  'e'   │
   │  capacité │  11  │              │  2   │  'l'   │
   └───────────┴──────┘              │  3   │  'l'   │
          world                      │  4   │  'o'   │
   ┌───────────┬──────┐              │  5   │  ' '   │
   │  ptr      │  ────┼───────────►  │  6   │  'W'   │
   │  taille   │  5   │              │  7   │  'o'   │
   └───────────┴──────┘              │  8   │  'r'   │
                                     │  9   │  'l'   │
                                     │ 10   │  'd'   │
                                     └──────┴────────┘
```

Nous créons des slices en utilisant un intervalle entre crochets en spécifiant `[indice_debut..indice_fin]`. Cependant,
il existe différentes manières de raccourcir cette écriture :

```rust
let s = String::from("hello");
let taille = s.len();

// Si indice_debut = 0 => optionel
let slice = &s[0..2];
let slice = &s[..2]; 

// Si indice_fin = taill => optionel
let slice = &s[3..taille];
let slice = &s[3..]; 

// Et par conséquent, si on combine les 2...
let slice = &s[0..taille];
let slice = &s[..]; 
```

> Remarque :
> 
> Les indices de l'intervalle d'une slice doivent toujours se trouver dans les zones acceptables de séparation des 
> caractères encodés en UTF-8. Si vous essayez de créé une slice qui s'arrête au milieu d'un caractère encodé sur 
> plusieurs octets, votre programme va se fermer avec une erreur
> 
> 
> ```rust
> fn main() {
>     let hello = "Hëllo";  // 'ë' est codé sur 2 octets en UTF-8
>     let slice = &hello[0..2];  // PANIQUE !
>     println!("{}", slice);
> }
> ```
> 
> Dans cet exemple, on a une chaine de 4 caractères (`"Hëllo"`), mais de 5 octets ! En coupant à l'indice 2 on coupe
> le caractère UTF-8 `ë` en deux et le programme plante ! 
> 
> ```
> thread 'main' (37022) panicked at src/main.rs:7:23:
> end byte index 2 is not a char boundary; it is inside 'ë' (bytes 1..3) of `Hëllo`
> ```
> ```
>   0     1    2      3   4   5
> ┌───┬──────┬──────┬───┬───┬────┐
> │ H │ C3   │ AB   │ l │ l │ o  │
> └───┴──────┴──────┴───┴───┴────┘
>     └─────────────┘
>            ë
> ```



## Les slices de chaines de caractères en paramètres

Savoir que l'on peut utiliser des slices de littéraux et de String nous incite à apporter une petite amélioration à 
`premier_mot()`, dont voici la signature :

```rust
fn premier_mot(s: &String) -> &str
```

On pourrait l'améliorer ainsi :

```rust
fn premier_mot(s: &str) -> &str
```

De cette façon, la fonction peut être appelée avec les `&String` et les `&str`.

```rust
fn main() {
    let ma_string = String::from("hello world");
    let mot = premier_mot(&ma_string[0..6]);
    let mot = premier_mot(&ma_string[..]);
    let mot = premier_mot(&ma_string);
    
    let mon_litteral_de_chaine = "hello world";
    let mot = premier_mot(&mon_litteral_de_chaine[0..6]);
    let mot = premier_mot(&mon_litteral_de_chaine[..]);
    let mot = premier_mot(&mon_litteral_de_chaine);
}
```

## Les slices ne sont pas que pour les chaines de caractères !

Il est tout à fait possible de faire des slices sur d'autres objets que les chaines de caractères en rust. En particulier,
sur les tableaux :

```rust
let tableau = [1, 2, 3, 4, 5];
let slice = &tableau[1..3];
assert_eq!(slice, &[2, 3]);
```


