use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

struct Anime {
  name: &'static str,
  bechdel_pass: bool,
}

fn main() {
  println!("Murderous Renaissance Artists");

  let mut table = Table::new();
  table.insert(
    "Gesualdo".to_string(),
    vec![
      "many madrigals".to_string(),
      "Tenebrae Responsoria".to_string(),
    ],
  );
  table.insert(
    "Caravaggio".to_string(),
    vec![
      "The Muscisians".to_string(),
      "The Calling of St. Matthew".to_string(),
    ],
  );
  table.insert(
    "Cellini".to_string(),
    vec![
      "Perseus with the head of Medusa".to_string(),
      "a salt cellar".to_string(),
    ],
  );

  borrowed_show(&table);
  assert_eq!(table["Gesualdo"][0], "many madrigals");

  sort_works(&mut table);

  let aria = Anime {
    name: "Aria: The Animation",
    bechdel_pass: true,
  };
  let anime_ref = &aria;
  // . operator implicit conversion reminder
  assert_eq!(anime_ref.name, "Aria: The Animation");
  assert_eq!((*anime_ref).name, "Aria: The Animation");
  assert_eq!((*anime_ref).bechdel_pass, true);

  show(table);
  // table gets moved into show, thus leaving the main instance uninitialized
  // comment in the assert to see the compiler (VSCode) complain
  // assert_eq!(table["Gesualdo"][0], "many madrigals");

  // references to references
  struct Point {
    x: i32,
    y: i32,
  }
  let point = Point { x: 1000, y: 729 };
  let r: &Point = &point;
  let rr: &&Point = &r;
  let rrr: &&&Point = &rr;
  assert_eq!(rrr.y, 729);
  assert_eq!(rr.x, 1000);
  assert!(std::ptr::eq(r, *rr));

  // borrowing refernces
  let r = &factorial(6);
  assert_eq!(r + &1009, 1729);
}

fn borrowed_show(table: &Table) {
  for (artist, works) in table {
    println!("works by {}:", artist);
    for work in works {
      println!("  {}", work);
    }
  }
}

fn show(table: Table) {
  for (artist, works) in table {
    println!("works by {}:", artist);
    for work in works {
      println!("  {}", work);
    }
  }
}

fn sort_works(table: &mut Table) {
  for (_artist, works) in table {
    works.sort();
  }
}

fn factorial(n: usize) -> usize {
  (1..n + 1).fold(1, |a, b| a * b)
}
