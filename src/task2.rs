#[test]

/*

use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // Tips: one of the approaches is to use `collect` method
    let teams_map2...

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}
*/


fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // Перший підхід: використання методу `collect`
    let teams_map2: HashMap<_, _> = teams.iter().cloned().collect();

    //Другий підхід:
    // let teams_map2: HashMap<_, _> = HashMap::from(teams);

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}
use std::collections::HashMap;

/*
У першому підході використовується метод collect(), щоб перетворити ітератор у HashMap.
У другому підході (який закоментований) використовується метод HashMap::from(), щоб створити хеш-мапу напряму з масиву.
*/