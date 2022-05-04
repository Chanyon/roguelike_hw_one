use rltk::{RandomNumberGenerator, Rltk, RGB};
use crate::{apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel};

use super::Rect;

// 地图（墙壁和地板）
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}
// 生成地图
/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

// 创建新房间
pub fn new_map_rooms_and_corridors() -> (Vec<Rect>,Vec<TileType>) {
    let mut map = vec![TileType::Wall;80*50];
    let mut rooms:Vec<Rect> =Vec::new();
    const MAX_ROOMS:i32 =  30;
    const MIN_SIZE:i32 = 3;
    const MAX_SIZE:i32 = 10;
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..MAX_ROOMS {
        let w = rng.roll_dice(MIN_SIZE,MIN_SIZE);
        let h = rng.roll_dice(MIN_SIZE,MAX_SIZE);
        let x = rng.roll_dice(1,80-w-1) - 1;
        let y = rng.roll_dice(1, 50-h-1) - 1;
        let new_room = Rect::new(x,y,w,h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }
        if ok {
            apply_room_to_map(&new_room,&mut map);
            if !rooms.is_empty() {
                let (new_x,new_y) = new_room.center();
                let (prev_x,prev_y) = rooms[rooms.len()-1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map,prev_x,new_x,prev_y);
                    apply_vertical_tunnel(&mut map,prev_y,new_y,new_x);
                } else {
                    apply_vertical_tunnel(&mut map,prev_y,new_y,prev_x);
                    apply_horizontal_tunnel(&mut map,prev_x,new_x,new_y);
                }
            }
            rooms.push(new_room);
        }
    }
    (rooms,map)
}
// 绘制地图
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile {
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('#'),
                );
            }
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('.'),
                );
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
