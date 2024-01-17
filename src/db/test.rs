enum Race {
    white(String),
    black(String),
}

struct Person {
    name: String,
    race: Race,
}

let skin = Race::white("white".to_string());