#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::new().add_startup_system(hello_world).run();
}

#[derive(Component)]
struct Test1;

fn hello_world() {
    println!("Hello World!");
}