use std::collections::HashMap;

struct Transition {
    target_scene: usize,
    action_text: String,
}

struct Scene {
    id: usize,
    text: String,
    transitions: Vec<Transition>,
}

struct Story {
    scenes: HashMap<usize, Scene>,
    start_scene: usize,
}

fn main() {
    println!("Hello, world!");
}
