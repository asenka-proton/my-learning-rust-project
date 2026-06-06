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
    println!("{}, s");
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
    │  ptr ─────────────────┼────────►│  'h' 'e' 'l' 'l' 'o' [vide]...│
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
    │  ptr ─────────────────┼────────►│  'h' 'e' 'l' 'l' 'o' [vide]...│
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

_Que se passe-t-il à la fin du programme (arrivé à `}`) ?_

Nous avons vu plus haut, qu'à ce moment, Rust appelle la fonction `drop` sur les variables allouées dans le tas. Cela 
va supprimer l'espace alloué dans le tas et désallouer dans la pile la structure de `s1` et `s2`. Or `s1`
et `s2` pointent vers **le même espace mémoire**. Pour régler ce problème, Rust va partir du principe qu'après la 
ligne `let s2 = s1`, `s1` ne sera plus en vigueur. Ainsi le code suivant : 

```rust
{
    let s1 = String::from("hello"); 
    let s2 = s1;
    println!("{}", s1);
}   
```

Produira une **erreur** :

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

En Rust, l'instruction `let s2 = s1`, est appelée un **déplacement**. On dira que `s1` a été déplacé dans `s2`. La véritable
forme du schéma précédent est donc :

```
          [ PILE / STACK ]                    [ TAS / HEAP ]
    ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐         ┌───────────────────────────────┐
           Structure s1               │      Tableau de caractères    │
    │       [invalidé]      │         │                               │
       ptr ─────────────────────────► │  'h' 'e' 'l' 'l' 'o' [vide]...│
    │  len = 5              │         │                               │
       cap = N                        │                               │
    └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘         │                               │
                                      └───────────────────────────────┘
    ┌───────────────────────┐                 ▲
    │      Structure s2     │                 │
    │      [en vigueur]     │                 │
    │  ptr ─────────────────┼─────────────────┘
    │  len = 5              │
    │  cap = N              │
    └───────────────────────┘
```

Plus de problème lors de l'appel au drop à la fin de la portée : il n'y a que le drop sur `s2` à réaliser, car `s1` n'est déjà
plus en vigueur !

Cela traduit aussi un choix de conception du langage Rust : il ne va jamais créer automatique une copie "profonde" des 
données. Par conséquent, toute copie automatique peut être considérée comme peu coûteuse en termes de performance.

## Le clonage

Pour faire explicitement une copie profonde des données sur le tas d'une `String` (et pas uniquement des données sur la pile),
il est possible d'utiliser la méthode `clone()`.

```rust
    let s1 = String::from("hello"); 
    let s2 = s1.clone();
    println!("{}", s1);
```

Cet extrait de code ne pose aucun problème lors de la compilation et fonctionnera parfaitement. Cependant, ce type d'opérations
est plus coûteuse. 

## La copie des données uniquement sur la pile

Le code suivant semble contredire ce que l'on vient de voir, car il fonctionne parfaitement :

```rust
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);
```

Nous n'avons pas appelé `x.clone()` et il n'y a pourtant aucun problème. La raison est que pour les variables stockées dans
la pile (comme les entiers, les réels, les booléens…), la copie est très rapide. Donc pas de différence entre la copie superficielle 
et la copie profonde. Appeler `clone()` devient donc inutile.

## Les traits `Copy` et `Drop`

Si un type implémente le trait `Copy` (comme les entiers, les réels, les booléens…) alors une variable sera toujours en
vigueur après avoir été affecté à une autre variable.

Si un type implémente le trait `Drop`, c'est l'inverse et le type se comportera comme les `String` que nous avons vu plus haut.

> À noter :
> 
> Le compilateur Rust n'autorise pas à annoter un type (une structure) avec le trait `Copy` si ce type ou un de ses membres
> implémente le type `Drop`.

Voici quelques exemples de types qui implémentent le trait `Copy` :

- tous les types entiers (comme `u32`),
- le type booléen `bool`
- tous les types flottants (comme `f64`),
- le type caractère, `char`,
- Les tuples, mais uniquement s'ils sont composés de type implémentant le trait `Copy`. Par exemple : `(i32, i32)`.


## La possession et les fonctions

Passer une variable à une fonction est similaire à une assignation de variable tel que vu précédemment : cela va
déplacer ou copier la variable dans la fonction. 

```rust
fn main() {
    
    let s = String::from("hello");  // `s` entre dans la portée (String -> Drop)
    prendre_possession(s);          // La valeur de `s` est déplacée dans la fonction...
                                    // ... et n'est plus en vigueur à partir d'ici !
                                    // La mémoire associée à `s` a déjà été libérée.
    
    let x = 5;                  // `x` entre dans la portée (i32 -> Copy)
    creer_copie(x);             // `x` va être déplacée dans la fonction, mais c'est un i32.
    println!("[main] x={}", x); // i32 implémente le trait Copy, donc x reste utilisable
    
}   // Ici `x` sort de la portée donc sa valeur est supprimée de la pile. C'est aussi le cas
    // de `s` mais comme sa valeur a été déplacée, rien ne se produit à ce niveau pour `s`

fn prendre_possession(txt: String) {    // `txt` entre dans la portée avec la valeur que la fonction 
                                        // appelante lui a donnée (`s`)
    println!("{}", txt);
} // Ici, `txt` sort de la portée, la fonction String.drop() est appelée pour libérer la mémoire.

fn creer_copie(n: i32) { // `n` entre dans la portée
    println!("{}", n);
} // Ici n sort de la portée, il est retiré de la pile, mais cela n'a pas d'impact sur le `x` de main() 
  // car la valeur a été intégralement copiée.
```

Si on avait essayé d'utiliser la variable `s` après l'instruction `prendre_possession(s);`, Le programme n'aurait pas 
compilé (_Value used after being moved [E0382]_).

**Retourner des valeurs** peut aussi transférer leur possession. Voyons cela encore avec un exemple concret : 

```rust
fn main() {
    
    let s1 = donner_possession();       // Déplace la valeur retournée par donner_possession()
                                        // dans la variable `s1` (String).
    
    let s2 = String::from("hello");     // `s2` entre dans la portée
    
    let s3 = prendre_et_rendre(s2);     // `s2` est déplacée dans prendre_et_rendre()
                                        // à la fin, la fonction va rendre la valeur qui
                                        // sera assignée (déplacée) dans `s3`
} // Ici, `s3` sort de la portée et la mémoire associée est libérée. 
  // `s2` a été déplacée, donc il ne se passe rien pour elle.
  // `s1` sort de la portée et sa mémoire est également libérée.

fn donner_possession() -> String {
    let txt = String::from("JuJu");     // `txt` entre dans la portée
    txt         // `txt` est retournée et est déplacée dans la fonction appelante
}

// La fonction prend un String en entrée et retourne ce même String
fn prendre_et_rendre(txt: String) -> String {   // `txt` entre dans la portée
    txt         // `txt` est retournée et est déplacée dans la fonction appelante
}
```

La possession d'une variable suit toujours le même schéma : assigner une valeur à une autre variable la **déplace**. 
Quand une variable qui contient des données sur le tas sort de la portée, la valeur sera nettoyée avec `drop` à moins
que la possession de cette donnée soit donnée à une autre variable.

La question qui se pose maintenant est : **comment faire en sorte de passer une valeur à une fonction, mais que cette
dernière n'en prenne pas possession ?** Il serait possible de renvoyer la valeur à chaque fois (comme avec `prendre_et_rendre()`),
mais cela peut vite devenir compliqué et peu lisible.

Pour cela Rust propose une fonctionnalité qui s'appelle les **références**.

