#[test]

/*

// FILL in the blanks and FIX the errors
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69.0);
    scores.insert("Katie", "58");

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"];
        assert_eq!(score, __);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), __);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}
*/



fn main() {
    let mut scores: HashMap<&str, f64> = HashMap::new(); // Указываем, что ключи будут строками, а значения - f64
    scores.insert("Sunface", 98.0); // Заменяем 98 на 98.0 для соответствия типу
    scores.insert("Daniel", 95.0);  // Заменяем 95 на 95.0 для соответствия типу
    scores.insert("Ashley", 69.0);
    scores.insert("Katie", 58.0);   // Заменяем "58" на 58.0 для соответствия типу

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98.0)); // Обратите внимание, что нужно сравнивать с ссылкой

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"];
        assert_eq!(score, 95.0); // Здесь мы сравниваем с 95.0
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3); // После удаления "Daniel" должно остаться 3 элемента

    for (name, score) in &scores { // Нужно использовать ссылку, чтобы избежать перемещения значений
        println!("The score of {} is {}", name, score);
    }
}

use std::collections::HashMap;