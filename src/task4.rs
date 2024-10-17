#[test]

/*

// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
use std::collections::HashMap;

struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
*/

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

/*
Щоб виправити помилки, потрібно реалізувати трейт Debug та Eq для структури Viking,
а також імплементувати трейт Hash, щоб структура могла використовуватися як ключ у хеш-таблиці HashMap

Додано #[derive(Debug, Eq, PartialEq, Hash)] для структури Viking,
що дозволяє її використовувати як ключ у HashMap, а також виводити її через println!("{:?}", ...).
Eq та PartialEq необхідні для того, щоб об'єкти можна було порівнювати, а Hash – для того, щоб використовувати їх у хеш-таблиці.
*/