extern crate ggez;
extern crate nalgebra;
extern crate skunkworks;

use ggez::{conf, event, Context, GameResult};

use ggez::graphics;
use ggez::graphics::{DrawMode, Font, Mesh, Point2, Text};

use nalgebra::Vector2;

use std::{env, path};

use skunkworks::game_timer::GameTimer;
use skunkworks::{draw_player, draw_waypoints, update_player, Actor, Waypoint};

pub struct MainState {
    player: Actor,
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
                label: Text::new(ctx, "1", &font)?,
            },
            Waypoint {
                position: Vector2::new(280.0, 250.0),
                label: Text::new(ctx, "2", &font)?,
            },
            Waypoint {
                position: Vector2::new(230.0, 440.0),
                label: Text::new(ctx, "3", &font)?,
            },
            Waypoint {
                position: Vector2::new(520.0, 510.0),
                label: Text::new(ctx, "4", &font)?,
            },
            Waypoint {
                position: Vector2::new(680.0, 100.0),
                label: Text::new(ctx, "5", &font)?,
            },
        ];

        let player = Actor {
            position: Vector2::new(20.0, 20.0),
            speed: 100.0,
            waypoints,
            destination_waypoint: 0,
        };

        let s = MainState {
            player,
            // font,
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
        // graphics::set_color(ctx, (0, 0, 255).into())?;
        draw_waypoints(ctx, &self.player.waypoints, &self.circle_mesh)?;
        draw_player(ctx, &self.player, &self.circle_mesh)?;

        graphics::present(ctx);
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
