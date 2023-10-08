use std::thread;

use crate::race::races::Races;
use crate::race::reader::read;
use crate::event::event_consumer::consume_uber;
use crate::event::event_producer::listen;

mod race;
mod event;

fn main() {
    let configuration_path: &str = "./tests/races.json";
    let configuration: Races = read(configuration_path);

    thread::spawn(|| {
        consume_uber()
    });

    listen(&configuration)
        .expect("TODO: panic message");
}
