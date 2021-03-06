extern crate ggez;
extern crate nalgebra;
extern crate skunkworks;

use ggez::{conf, event, Context, GameResult};

use ggez::graphics;
use ggez::graphics::{DrawMode, Font, Mesh, Point2};

use nalgebra::Vector2;

use std::{env, path};

use skunkworks::game_timer::GameTimer;
use skunkworks::{actor_at_waypoint, draw_player, draw_waypoint_labels, draw_waypoints,
                 move_towards_next_waypoint, Actor, Waypoint};

pub struct MainState {
    player: Actor,
    font: Font,
    circle_mesh: Mesh,
    game_timer: GameTimer,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = Font::new(ctx, "/font.ttf", 12)?;
        let circle_mesh = Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 14.0, 0.4)?;

        let waypoints = vec![
            Waypoint {
                position: Vector2::new(120.0, 30.0),
            },
            Waypoint {
                position: Vector2::new(280.0, 250.0),
            },
            Waypoint {
                position: Vector2::new(230.0, 440.0),
            },
            Waypoint {
                position: Vector2::new(520.0, 510.0),
            },
            Waypoint {
                position: Vector2::new(680.0, 100.0),
            },
        ];

        let player = Actor {
            position: Vector2::new(20.0, 20.0),
            speed: 100.0,
            waypoints,
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
        move_first_to_last(&mut player.waypoints);
    }
    Ok(())
}

pub fn move_first_to_last<T>(list: &mut Vec<T>) {
    if !list.is_empty() {
        let first: T = list.remove(0);
        list.push(first);
    }
}
