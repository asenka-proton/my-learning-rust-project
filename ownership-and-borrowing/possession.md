# La possession en Rust

Il y a souvent 2 approches de la gestion de la mémoire par les différents langages de programmation :

- Allocation et dés-allocation explicite des espaces mémoire par le programmeur (C, C++, ...),
- Utilisation d'un ramasse-miettes (Java).

Rust prend une troisième voie : la mémoire est gérée par un **système de possession** qui repose sur un ensemble de **règles
strictes**. Ces règles sont vérifiées au moment de la **compilation** ; si une des règles est enfreinte, le programme ne
compilera pas. Et cela se fait sans jamais ralentir l'exécution du programme (comme peut le faire un ramasse-miettes).

## Les règles de la possession

**A retenir:**

1. Chaque valeur en Rust a une **variable** que l'on appelle son **propriétaire**.
2. Il ne peut y avoir qu'**un seul propriétaire à la fois**.
3. Quand le propriétaire **sort de la portée**, la valeur sera **supprimée**.

## La portée de la variable

La portée est une zone d'un programme dans laquelle un élément est "en vigueur". 

```rust
{                       // `s` n'est pas en vigueur ici
    let s = "hello";    // `s` entre dans la portée
    
    // On peut faire des choses avec `s` ici
    // ...

}   // la portée de `s` est terminée, `s` n'est plus en vigueur et sa valeur est supprimée.        
```

Deux étapes importantes dans l'exemple ci-dessus :

1. Quand `s` **entre dans la portée** (`s` devient en vigueur).
2. Cela reste ainsi jusqu'à ce qu'elle sorte de la portée.

> **Note :**
> 
> Jusqu'ici, rien de nouveau par rapport à la plupart des langages de programmation : une variable est utilisable dans
> le bloc de code dans lequel il a été créé (sauf avec cette daube de Javascript qui permet des dingueries sans aucun sens).

## Utiliser le type `String` pour mieux comprendre

Pour mieux illustrer les règles de possession et les particularités de Rust, il nous faut un type
de données plus complexe (différent de types classiques comme `i32`, `usize`, `f64`, `&str`, ...).
Ces types ont la particularité d'avoir toujours une **taille connue** et peuvent être stockée dans la 
**pile** (stack). Ils sont retirés de la pile lorsque la portée n'en a plus besoin. Ainsi, ils sont
rapidement et facilement **copiés**. Pour mieux expérimenter le stockage de données dans le **tas** (heap),
le type **String** est un bon exemple. Cependant, cela peut s'appliquer à bien d'autres types de 
données (créé par le programmeur ou présent dans la bibliothèque standard).

Les littéraux de chaines de caractères (`&str`) sont pratiques quand une chaine est codée en dur dans
le programme. Mais ils ne conviennent pas quand on veut stocker une chaine venant d'une saisie utilisateur
par exemple (en gros, si on ne connait pas son contenu exact avant l'exécution). Dans ce cas, Rust
propose le type standard `String`. Avec ce type, les données sont stockées **sur le tas** (heap) et pas 
dans la pile. On peut créer un String à partir d'un littéral de chaine comme ceci:

```rust
    let s = String::from("hello");
```

Ce type de chaine de caractères peut être mutable (mais pas obligatoirement) :

```rust
{
    let mut s = String::from("hello");
    s.push_str(", world!");
    println !("{}, s");
}
```

La raison pour laquelle `String` peut être mutable et pas les littéraux de chaines de caractères
se trouve dans la manière dont ces 2 types travaillent avec la mémoire.

Quand nous appelons `String::from("hello")`, le gestionnaire de mémoire va allouer un espace mémoire
adéquat lors de l'exécution du programme. La difficulté est donc de savoir **comment libérer cet espace**.

Dans un langage avec un ramasse-miettes (comme Java) : rien à faire. Le ramasse-miette fait tout le 
boulot. Dans un langage comme le C ou le C++, c'est au développeur de savoir quand et comment
désallouer cet espace mémoire. On sait depuis longtemps que c'est là que se trouve la difficulté 
et que beaucoup de bugs et de faille de sécurité viennent de la difficulté de cette opération.En Rust,
la mémoire sera libérée dès que la variable qui la possède sort de la portée.

```rust
{
    let s = String::from("hello"); // `s` entre dans la portée

    // On peut faire des choses avec `s`
}   // `s` sort de la portée et n'est plus en vigueur => appelle à la fonction drop()
```

Quand `s` sort de la portée, Rust appelle une fonction appelée `drop`. `String` "implémente" donc cette
méthode pour pouvoir correctement libérer la mémoire. 

Cela peut sembler simple, mais quand le code se complexifie cela peut avoir des implications inattendues
si l'on est habitué à d'autres langages de programmation.


## Intéraction entre les variables

Prenons cet exemple :

```rust
{
    let x = 5; // Assigner la valeur 5 (i35) à la variable `x`
    let y = x; // On fait une copie de la valeur de `x` dans la nouvelle variable `y`
    // `x` et `y` sont stockée dans la pile
}   
```

Maintenant avec cet autre exemple, les choses sont différentes :

```rust
{
    let s1 = String::from("hello"); 
    let s2 = s1;
}   
```

Pour mieux comprendre ce qu'il se passe, voyons comment est structurée la `String` `s1` :

```
          [ PILE / STACK ]                    [ TAS / HEAP ]
    ┌───────────────────────┐         ┌───────────────────────────────┐
    │      Structure s1     │         │      Tableau de caractères    │
    │                       │         │                               │
    │  ptr ────────────────►│────────►│  'h' 'e' 'l' 'l' 'o' [vide]...│
    │  len = 5              │         │                               │
    │  cap = N              │         │                               │
    └───────────────────────┘         │                               │
                                      └───────────────────────────────┘
```

A la fin de l'exécution de l'instruction `let s1 = String::from("hello")`, voici la situation (nous
pouvons ignorer la notion de capacité pour la compréhension de cette section).

Lors de l'exécution `let s2 = s1`, c'est bien les données de s1 qui sont copiés : donc le **pointeur**
vers le tableau de caractères qui se trouve dans le tas.

```
          [ PILE / STACK ]                    [ TAS / HEAP ]
    ┌───────────────────────┐         ┌───────────────────────────────┐
    │      Structure s1     │         │      Tableau de caractères    │
    │                       │         │                               │
    │  ptr ────────────────►│────────►│  'h' 'e' 'l' 'l' 'o' [vide]...│
    │  len = 5              │         │                               │
    │  cap = N              │         │                               │
    └───────────────────────┘         │                               │
                                      └───────────────────────────────┘
    ┌───────────────────────┐                 ▲
    │      Structure s2     │                 │
    │                       │                 │
    │  ptr ─────────────────┼─────────────────┘
    │  len = 5              │
    │  cap = N              │
    └───────────────────────┘
```

On voit bien, que les données utiles (le tableau de caractères), n'a pas été copié. À la place, 
on a 2 structures `s1` et `s2` qui contiennent un pointeur vers le tableau situé dans le tas.

Qu