// Exercice écrit par N. Matsakis (2016)
fn main() {
  let (adjective, name) = two_words();
  let name = format!("{} {}", adjective, name);
  print_out(name);
}

fn two_words() -> (String, String) {
  (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: &String) -> String {
  // But #1: Que faire pour que ça compile? 
  //
  // -> On ajoute `mut` car output doit être mutable.
  let mut output = String::new();
  for c in name.chars() {
      match c {
          'a' | 'e' | 'i' | 'o' | 'u' => {
              // on saute les voyelles
          }
          _ => {
              output.push(c);
          }
      }
  }
  output
}

fn print_out(name: String) {
  //let name2 = name.clone();
  //let devowelized_name = remove_vowels(name2);
  let devowelized_name = remove_vowels(&name);
  println!("Removing vowels yields {:?}", devowelized_name);

  // But #2: Que se passe-t-il vous décommentez le `println` suivant?
  // Pouvez-vous changer le code qui précède pour que le code suivant compile?
  //
  // -> On clone la variable passée en paramètre `name`à l'aide de la commande `clone()` et on 
  //    passe la variable clonée en paramètre
  println!("Removing vowels from {:?} yields {:?}", name, devowelized_name);

  // Bonus: Pouvez-vous le faire sans copier de données?
  // (En utilisant uniquement le transfert de possession)
  //
  // -> On passe la possession de la variable `name`à la méthode en la passant par @
  //    à la méthode `remove_vowels()`
}
