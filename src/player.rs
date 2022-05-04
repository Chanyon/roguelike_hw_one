use rltk::{VirtualKeyCode, Rltk};
use crate::{try_move_player, State};

// 玩家控制移动
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
  match ctx.key {
      Some(key) => match key {
          VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
          VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
          VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
          VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
          _ => {}
      },
      None => {}
  }
}