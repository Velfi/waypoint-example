extern crate ggez;
extern crate nalgebra;
extern crate skunkworks;

use ggez::{conf, event, Context, GameResult};
use ggez::event::MouseButton;

use ggez::graphics;
use ggez::graphics::{DrawMode, Font, Mesh, Point2};

use nalgebra::Vector2;

use std::{env, path};

use skunkworks::game_timer::GameTimer;
use skunkworks::{actor_at_waypoint, draw_player, draw_waypoints, draw_waypoint_labels, move_towards_next_waypoint,
                 Actor, Waypoint};

pub struct MainState {
    player: Actor,
    circle_mesh: Mesh,
    game_timer: GameTimer,
    font: Font,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = Font::new(ctx, "/font.ttf", 12)?;
        let circle_mesh = Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 14.0, 0.4)?;

        let player = Actor {
            position: Vector2::new(20.0, 20.0),
            speed: 200.0,
            waypoints: Vec::with_capacity(5),
        };

        let s = MainState {
            player,
            font,
            circle_mesh,
            game_timer: GameTimer::new(),
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        if let MouseButton::Right = button {
            let new_waypoint = Waypoint::new(x as f32, y as f32);
            self.player.waypoints.push(new_waypoint);
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.game_timer.tick();
        update_player(&mut self.player, &self.game_timer.get_frame_time())?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_waypoints(ctx, &self.player.waypoints, &self.circle_mesh)?;
        draw_waypoint_labels(ctx, &self.font, &self.player.waypoints)?;
        draw_player(ctx, &self.player, &self.circle_mesh)?;

        graphics::present(ctx);

        if (self.game_timer.get_ticks() % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Test", "Waypoint", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

pub fn update_player(player: &mut Actor, frame_time: &f64) -> GameResult<()> {
    move_towards_next_waypoint(player, frame_time);
    if actor_at_waypoint(player) {
        player.waypoints.remove(0);
    };
    Ok(())
}
