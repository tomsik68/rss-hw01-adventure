use std::collections::HashMap;

struct Transition {
    target_scene: String,
    action_text: String,
}

struct Scene {
    id: String,
    text: String,
    transitions: Vec<Transition>,
}

struct Story {
    scenes: HashMap<String, Scene>,
}

fn main() {
    println!("Hello, world!");
}
