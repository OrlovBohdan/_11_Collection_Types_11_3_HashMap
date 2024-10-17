#[test]

/*
// FIX the errors with least changes
// DON'T remove any code line
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello".to_string();
  let mut m2 = HashMap::new();
  // Ownership moved here
  m2.insert(v2, v1);

  assert_eq!(v2, "hello");

  println!("Success!");
}
*/

use std::collections::HashMap;
fn main() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 досі можна використовувати після вставки в hashmap: {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Використовуємо посилання тут, щоб уникнути переміщення власності
    m2.insert(&v2, v1);

    // Порівнюємо без змін, оскільки власність не переміщувалась
    assert_eq!(v2, "hello");

    println!("Успіх!");
}

/*
m2.insert(&v2, v1); тепер вставляє посилання на v2, що дозволяє використовувати v2 після вставки.
assert_eq!(v2, "hello"); залишається незмінним, оскільки власність на v2 не переміщується.
*/