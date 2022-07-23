use rltk::{GameState, Rltk, RGB,Point};
use std::cmp::{max, min};
use specs::prelude::*;

mod component;
mod map;
mod rect;
mod player;
mod visibility_system;
mod monster_ai_system;
mod map_indexing_system;
use visibility_system::VisibilitySystem;
use component::{Position,Player,Renderable,Viewshed,Monster,Name,BlocksTile};
use map::*;
use rect::Rect;
use player::player_input;
use monster_ai_system::MonsterAI;
use map_indexing_system::MapIndexingSystem;

// player.rs
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut ppos = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();
    
    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x+delta_x,pos.y+delta_y);
        // 玩家移动时更新资源
        ppos.x = pos.x;
        ppos.y = pos.y;
        // 如果遇到墙不能再移动
        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum RunState{
    Paused,Running //游戏要么在运行，要么等待输入
}
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        }else{
            self.runstate = player_input(self,ctx);
        }

        draw_map(&self.ecs,ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            // 可见区域
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] { 
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);
        self.ecs.maintain();
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new(), runstate: RunState::Running};
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    // 注册组件
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
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
    .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true})
    // .with(Name { name: "Player".to_string()})
    .build();

    // 怪物
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i,room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();
        let glyph:rltk::FontCharType;
        let name:String;
        let roll = rng.roll_dice(1, 2);
        
        match roll {
            1 => {glyph = rltk::to_cp437('g'); name = "Goblin".to_string();},
            _ => {glyph = rltk::to_cp437('o'); name = "Orc".to_string();}
        }
        gs.ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable{
            glyph,
            bg: RGB::named(rltk::RED),
            fg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed{ visible_tiles: Vec::new(), range:8,dirty: true})
        .with(Monster{})
        .with(Name { name: format!("{} #{}",name,i)})
        .with(BlocksTile{})
        .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x,player_y));
    rltk::main_loop(context, gs)
}

