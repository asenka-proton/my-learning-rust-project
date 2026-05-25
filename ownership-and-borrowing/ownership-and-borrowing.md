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

