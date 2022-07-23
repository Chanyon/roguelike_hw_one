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

// 视域
#[derive(Component,Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

//怪物组件
#[derive(Component,Debug)]
pub struct Monster {}

// 名字组件
#[derive(Component,Debug)]
pub struct Name{
    pub name: String,
}

#[derive(Component,Debug)]
pub struct BlocksTile {}