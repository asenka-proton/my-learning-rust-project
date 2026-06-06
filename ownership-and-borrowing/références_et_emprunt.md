# Références et emprunt

## Les références

Une référence est un moyen de fournir une adresse vers une valeur. Cela fonctionne un peu comme un **pointeur**. Cependant,
les références ont une différence notable avec les pointeurs : une référence **garantit** de pointer vers une valeur **en 
vigueur**. Voyons cela avec un premier exemple :

```rust
fn main() {
    
    let s1 = String::from("hello");
    
    let taille = calculer_la_taille_de(&s1); // On donne une référence, donc `s1` n'est PAS déplacée dans calculer_la_taille_de()
    
    println!("La taille de '{}' est {}", s1, taille); // On peut toujours utiliser `s1` !
}

fn calculer_la_taille_de(txt: &String) -> usize {
    txt.len()
}  // `txt` sort de la portée => pas d'impact sur `s1` dans main()
```

Ici, nous passons une référence à `s1` avec le symbole `&` à la fonction `calculer_la_taille_de()`. Cette dernière fonction
prend en paramètre une variable `txt` de type `&String` (comprendre _"référence vers un String"_).

On se retrouve avec ce schéma en mémoire :

```text
    ┌───────────────────────┐          ┌───────────────────────┐         ┌───────────────────────────────┐
    │           txt         │          │          s1           │         │      Tableau de caractères    │
    │       (&String)       │          │       (String)        │         │                               │
    │                       │          │                       │         │                               │
    │  ptr ─────────────────┼─────────►│  ptr ─────────────────┼────────►│  'h' 'e' 'l' 'l' 'o' [vide]...│
    └───────────────────────┘          │  len = 5              │         │                               │
                                       │  cap = N              │         └───────────────────────────────┘  
                                       └───────────────────────┘ 
```

On a bien ici, une référence `txt` qui pointe vers un `String` `s1` qui, lui-même, a un pointeur vers des données dans le 
tas avec le tableau de caractères. L'opération `&s1` a créé la référence, qui a été stockée dans `txt`, mais `txt` n'a pas pris 
possession des données de `s1` (qui reste toujours l'unique propriétaire).

**Conséquente importante** : quand `txt` sortira de la portée, les données de s1 ne seront **pas** libérées de la mémoire.

Autrement dit : lorsque les fonctions ont des références en paramètres au lieu des valeurs réelles, nous n'avons pas besoin de retourner 
les valeurs pour les rendre, car nous n'en avons jamais pris possession.

> **Remarque :**
> 
> L'opération inverse de la création de référence (`&`) est le **déréférencement**. Il s'effectue avec l'opérateur
> `*`. Il ne sera pas traité dans cette section.

## L'emprunt

On appelle également **"emprunt"** l'action de créer une référence. On peut **emprunter** une valeur en Rust, mais il 
faut la rendre ! Si on pousse l'idée un peu plus loin maintenant : **que se passe-t-il si on veut modifier une variable
empruntée** (une référence) ?

Prenons cet extrait de code (qui ne fonctionne PAS) : 

```rust
fn main() {
    let s = String::from("hello");
    changer(&s);
}
fn changer(txt: &String) {
    txt.push_str(", world!"); // error[E0596]: cannot borrow `*txt` as mutable, as it is behind a `&` reference
}
```

Il faut se rappeler ici, qu'en Rust, par défaut, les variables sont **immuables**. C'est aussi le cas pour les références !

### Les références mutables

Pour rendre l'extrait de code précédent fonctionnel, il faut utiliser une **référence mutable** (`&mut`):

```rust
fn main() {
    let mut s = String::from("hello");
    changer(&mut s);
}
fn changer(txt: &mut String) {
    txt.push_str(", world!"); // Maintenant ca marche !
}
```

Tout d'abord, il faut que `s` soit mutable (utilisation du mot clef `mut`), on le déclare donc avec 
`let mut s = ...`. Ensuite, il faut que le paramètre `txt` de la fonction `changer()` soit une référence 
mutable : `txt: &mut String`. Et enfin, lors de l'appel à la fonction `changer()`, nous créons une référence
mutable vers `s`: `&mut s`.

> **IMPORTANT !**
> 
> Les références mutables ont une importante contrainte : il ne peut exister qu'**une seule référence mutable 
> au même moment** !

Prenez par exemple ce morceau de code :

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // error[E0499]: cannot borrow `s` as mutable more than once at a time
println!("{}, {}", r1, r2);
```

Le problème est, qu'entre la création de `r1` et son utilisation dans le `println`, nous
avons créé une seconde référence mutable sur `s` appelée `r2`.

La limitation qui empêche d'avoir plusieurs références mutables vers la même donnée au même moment autorise 
les mutations, mais **de manière très contrôlée**. Cela peut être déroutant lorsque l'on est habitué à d'autres
langages (comme le Java) qui sont moins restrictif sur ce point. Mais l'intérêt de cette contrainte est que, de 
cette façon, Rust sait empêcher les **accès concurrents** dès la compilation !

> Définition : **accès concurrent**
> 
> Un accès concurrent est une situation de concurrence qui se produit lorsque ces trois facteurs se combinent :
> 1. Deux pointeurs (ou plus) accèdent à **la même donnée en même temps**,
> 2. Au moins un de ces pointeurs est utilisé pour **écrire** dans cette donnée,
> 3. Il n'y a **pas de mécanisme de synchronisation** des accès à la donnée.


On peut par contre avoir plusieurs références **non mutables** en même temps sans que cela pose de problème :

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    println!("{} et {}", r1, r2);
}
```

Par contre, impossible de combiner des références non mutables avec une référence mutable ! Cela peut paraitre 
étonnant de prime abord, mais c'est finalement assez logique : en effet, les références non-mutables ne s'attendent pas à ce que 
la valeur vers laquelle elles pointent puissent changer soudainement.

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // sans problème
    let r2 = &s; // sans problème
    let r3 = &mut s; // GROS PROBLEME
    println!("{}, {}, et {}", r1, r2, r3);
}
```

Toutefois, il y a encore une subtilité : la portée d'une référence commence dès qu'elle est **créée** et se poursuit
jusqu'au dernier endroit où elle est **utilisée**. Ainsi, le code suivant fonctionne parfaitement, car la dernière
utilisation de la référence immuable — le `println!` — est située **avant la création de la référence mutable** !   

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    println!("{} et {}", r1, r2);

    let r3 = &mut s; // sans problème !
    println!("{}", r3);
}
```

### Les références "pendouillantes"

Une "référence pendouillante" (ou "dangling pointer") est un pointeur qui pointe vers un emplacement mémoire qui  
a déjà été libéré. La force de Rust est justement de prévenir ce type de situation directement à la compilation. Voyons
un exemple qui essaie de créer une référence pendouillante (ce code ne compile pas) :

```rust
fn main() {
    let str = pendouille(); // la fonction renvoie une référence vers un String (&String)
}

fn pendouille() -> &String {      
    let s = String::from("hello");   // `s` entre dans la portée de la fonction pendouille()
    &s // On renvoie la référence vers `s`
}  // Ici, `s` sort de la portée et la mémoire associée est libérée
```

L'erreur ici fait référence à la notion de durée de vie (non traitée dans cette section). Le compilateur expliquera en 
complément : "_Le type de retour de cette fonction contient une valeur empruntée, mais il n'y a
plus aucune valeur qui peut être empruntée._" 

La solution dans ce cas précis serait que la fonctione `pendouille()` renvoie un `String` et pas une référence. De cette
façon la possession de la valeur renvoyée est donnée à la variable `str` dans la fonction `main()`.

## Les règles de référencement

Deux règles à retenir de ce chapitre :

1. A un instant donné, vous pouvez avoir :
   - soit, une seule référence **mutable**,
   - soit, un nombre quelconque de références **immuables**.
2. Les références doivent toujours être **en vigueur**.