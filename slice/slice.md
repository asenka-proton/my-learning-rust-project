# Le type Slice

Une _slice_ permet d'obtenir une référence vers une séquence continue d'éléments d'une collection plutôt que toute la 
collection.

## Un exemple de problème

Ecrire une fonction qui prend une chaine de caractères en retourne le premier mot dans cette chaine.

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

```
          s1
   ┌───────────┬──────┐              ┌──────┬────────┐
   │  ptr      │  ────┼───────────►  │  0   │  'H'   │
   │  taille   │  11  │              │  1   │  'e'   │
   │  capacité │  11  │              │  2   │  'l'   │
   └───────────┴──────┘              │  3   │  'l'   │
          s2                         │  4   │  'o'   │
   ┌───────────┬──────┐              │  5   │  ' '   │
   │  ptr      │  ────┼───────────►  │  6   │  'W'   │
   │  taille   │  5   │              │  7   │  'o'   │
   └───────────┴──────┘              │  8   │  'r'   │
                                     │  9   │  'l'   │
                                     │ 10   │  'd'   │
                                     └──────┴────────┘
```