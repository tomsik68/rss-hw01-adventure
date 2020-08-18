use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

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


fn parse_transitions(line: &str) -> Vec<Transition> {
    line.split(',')
        .map(|t| {
            let mut s = t.split(':');
            Transition {
                action_text: s.next().unwrap().trim().to_string(),
                target_scene: s.next().unwrap().trim().to_string(),
            }
        })
        .collect()
}

fn parse_story(filename: &dyn AsRef<Path>) -> io::Result<Story> {
    let f = File::open(filename)?;
    let br = BufReader::new(f);

    let mut scenes = HashMap::new();
    let mut scene_id = String::new();
    let mut scene_text = String::new();

    for (i, line) in br.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        match i % 3 {
            0 => {
                scene_id = line.to_string();
            }
            1 => {
                scene_text = line.to_string();
            }
            2 => {
                let transitions = parse_transitions(line);
                scenes.insert(
                    scene_id.clone(),
                    Scene {
                        id: scene_id,
                        text: scene_text,
                        transitions,
                    },
                );

                scene_id = String::new();
                scene_text = String::new();
            }
            _ => panic!("i % 3 is none of 0,1,2"),
        }
    }

    Ok(Story { scenes })
}

fn main() {
    let filename: String = env::args()
        .nth(1)
        .expect("Please supply filename of the story");

    let story = parse_story(&filename).expect("unable to parse story");
}
