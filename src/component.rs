use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub  glyph: rltk::FontCharType,
    pub bg: RGB,
    pub fg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}