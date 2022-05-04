use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}
// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }
#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    bg: RGB,
    fg: RGB,
}
#[derive(Component)]
struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);
    fn run(&mut self, data: Self::SystemData) {
        let (lefty, mut pos) = data;
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}
#[derive(Component, Debug)]
struct Player {}
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

// 玩家控制移动
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
        None => {}
    }
}

// 地图（墙壁和地板）
#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}
fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}
// 生成地图
fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor;80*50];
    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall;
        map[xy_idx(x,49)] = TileType::Wall;
    }
    for y in 0..50{
        map[xy_idx(0,y)] = TileType::Wall;
        map[xy_idx(79,y)] = TileType::Wall;
    }
    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..400{
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1,49);
        let idx = xy_idx(x,y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}
// 绘制地图
fn draw_map(map:&[TileType],ctx:&mut Rltk){
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile {
            TileType::Wall => {
                ctx.set(x, y,RGB::from_f32(0.0,1.0,0.0),RGB::from_f32(0.0,0.0,0.0), rltk::to_cp437('#'));
            },
            TileType::Floor => {
                ctx.set(x, y,RGB::from_f32(0.5,0.5,0.5) , RGB::from_f32(0.0,0.0,0.0), rltk::to_cp437('.'));
            },
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
struct State {
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
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    // 注册组件
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftWalker>();
    gs.ecs.register::<Player>();
    // 创建实体,它们只不过是一个标识号，告诉 ECS 存在一个实体
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                bg: RGB::named(rltk::RED),
                fg: RGB::named(rltk::BLACK),
            })
            .with(LeftWalker {})
            .build();
    }
    gs.ecs.insert(new_map());


    rltk::main_loop(context, gs)
}
