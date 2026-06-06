
pub fn copy_and_move() {
    {
        let x = 5;
        let y = x;
        println!("x={}, y={}", x, y);
    }

    {
        let x = 5;
        let y = x.clone(); // Fais la même chose que dans le bloc précédent (let y =x)
        println!("x={}, y={}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 n'est plus en vigueur à partir d'ici
        //println!("s1={}, s2={}", s1, s2); // Erreur: borrow of moved value: `s1`
        println!("s2={}", s2);
    }
}

pub fn ownership_and_functions() {
    let s = String::from("hello");
    prendre_possession(s);

    let x = 5;
    creer_copie(x);
    println!("[main] x={}", x);
}

fn prendre_possession(txt: String) {
    println!("{}", txt);
}

fn creer_copie(n: i32) {
    println!("{}", n);
}

pub fn multiple_reference_mutables() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}, {}", r1, "");
}
