fn main() {
    let hello = "Hëllo";  // 'ë' est codé sur 2 octets en UTF-8

    // ✗ ERREUR : Panique à l'exécution !
    // "Hëllo".len() = 5 octets, pas 5 caractères
    // Index 2 tombe AU MILIEU du caractère 'ë'
    let slice = &hello[0..2];  // PANIQUE !

    println!("{}", slice);
}
