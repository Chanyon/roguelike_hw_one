use rltk::{GameState, Rltk, RGB};
use std::cmp::{max, min};
use specs::prelude::*;

mod component;
mod map;
mod rect;
mod player;
use component::{Position,Player,Renderable};
use map::*;
use rect::Rect;
use player::player_input;


// #[derive(Component)]
// struct LeftWalker {}
// impl<'a> System<'a> for LeftWalker {
//     type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);
//     fn run(&mut self, data: Self::SystemData) {
//         let (lefty, mut pos) = data;
//         for (_lefty, pos) in (&lefty, &mut pos).join() {
//             pos.x -= 1;
//             if pos.x < 0 {
//                 pos.x = 79;
//             }
//         }
//     }
// }

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination = xy_idx(pos.x+delta_x,pos.y+delta_y);
        // 如果遇到墙不能再移动
        if map[destination] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map,ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        // let mut lw = LeftWalker {};
        // lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
// 制作房间
fn apply_room_to_map(room:&Rect,map:&mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x,y)] = TileType::Floor;
        }
    }
}
// 制作走廊
fn apply_horizontal_tunnel(map:&mut [TileType],x1:i32,x2:i32,y:i32) {
    for x in min(x1,x2)..=max(x1,x2) {
        let idx = xy_idx(x,y);
        if idx > 0 && idx < 80*50 {
            map[idx] = TileType::Floor;
        }
    }
}
fn apply_vertical_tunnel(map:&mut [TileType],y1:i32,y2:i32,x:i32) {
    for y in min(y1,y2)..=max(y1,y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    let (rooms,map) = new_map_rooms_and_corridors();
    let (player_x, player_y) = rooms[0].center();
    // 注册组件
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftWalker>();
    gs.ecs.register::<Player>();
    // 创建实体,它们只不过是一个标识号，告诉 ECS 存在一个实体
    gs.ecs
    .create_entity()
    .with(Position { x: player_x, y: player_y })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player {})
    .build();
    
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
