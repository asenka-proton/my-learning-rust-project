use crate::cell::Cell;

pub fn create_vector() {
    let v: Vec<i32> = Vec::new();
    dbg!(v);
}

pub fn create_vector_with_macro() {
    let v = vec![1, 2, 3, 4];
    dbg!(v);
} // <- Libérer un vecteur libère aussi ses éléments

pub fn update_vector() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.push(6);
    v.push(7);
    dbg!(v);

    /*
    Lorsque le vecteur est libéré, tout son contenu est aussi libéré,
    ce qui veut dire que les nombres entiers qu'il stocke vont être
    effacés de la mémoire.
     */
}

pub fn read_vector_value() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Le troisième élément est {}", third);

    match v.get(2) {
        Some(element) => println!("Le troisième élément est {}", element), // <=
        None => println!("Il n'y a pas de troisième élément"),
    }

    // v.get(i) renvoie une Option avec une REFERENCE vers l'élément (ici: &i32) ou Option::None
}

pub fn read_vector_value_index_out_of_bounds() {
    let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[100];  ERROR: panic : index out of bounds

    match v.get(100) {
        Some(element) => println!("Le centième élément est {}", element),
        None => println!("Il n'y a pas de centième élément"), // <=
    }
}

pub fn borrowing_error() {
    let mut v = vec![1, 2, 3, 4];

    let first = &v[0];

    // v.push(5); // Erreur: cannot borrow `v` as mutable because it is also borrowed as immutable
    // L"instruction précédente ne fonctionne pas car il existe, dans la même portée,
    // une référence empruntée (first) vers une valeur du vecteur./

    println!("le premier élément est {}", first);

    /*
    Cette erreur s'explique par la façon dont les vecteurs fonctionnent : comme les vecteurs
    ajoutent les valeurs les unes à côté des autres dans la mémoire, l'ajout d'un nouvel élément
    à la fin du vecteur peut nécessiter d'allouer un nouvel espace mémoire et copier tous les
    anciens éléments dans ce nouvel espace, s'il n'y a pas assez de place pour placer tous les
    éléments les uns à côté des autres dans la mémoire là où est actuellement stocké le vecteur.
    Dans ce cas, la référence au premier élément pointerait vers de la mémoire désallouée.
    Les règles d'emprunt évitent aux programmes de se retrouver dans cette situation.
     */
}

pub fn iterate_vector_values() {
    let v1 = vec![2, 50, 25, 10, -2];

    for element in &v1 {
        println!("{}", element);
    }

    let mut v2 = vec![2, 50, 25, 10, -2];

    for element in &mut v2 {
        *element += 100;
    }
    dbg!(v2);

    /*
    Afin de changer la valeur vers laquelle pointe la référence mutable, nous devons utiliser
    l'opérateur de déréférencement * pour obtenir la valeur dans element avant que nous puissions
    utiliser l'opérateur +=.
     */
}

pub fn use_vector_to_store_different_types_of_data() {
    let line = vec![
        Cell::Int(3),
        Cell::Text(String::from("bleu")),
        Cell::Float(10.12),
    ];

    for element in &line {
        println!("{:?}", element);
    }
}
