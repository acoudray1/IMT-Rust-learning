// Exercice écrit par N. Matsakis (2016)

pub fn main() {
  let string = format!("my friend");
  greet(&string);
  greet(&string);
}

fn greet(name: &String) {
  let slice = &name[3..];
  println!("Hello, {}!", slice);
}

// But #1: Convertir `greet` pour utiliser l'emprunt, et non la possession,
// pour que le programme s'exécute sans clonage (copie profonde).
//
// -> On ajoute comme dans ex01.rs le passage par adresse sur le parametre name.
//
// But #2: Utiliser une sous-tranche (subslice) pour que cela imprime
//  "Hello, friend" au lieu de "Hello, my friend".
//
// -> On utilise l'indexation des str pour modifier notre string directement : str[<index de suppression>..<index de suppression>]
