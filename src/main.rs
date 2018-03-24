extern crate ggez;
extern crate nalgebra;

use std::time::{
    Instant,
    Duration
};

use ggez::{
    conf,
    event,
    Context,
    GameResult
};

use ggez::graphics;
use ggez::graphics::{
    Color,
    DrawMode,
    Font,
    Mesh,
    Point2,
    Text
};

use nalgebra::{
    Vector2,
    distance,
    normalize
};

use std::{
    env,
    path
};

pub const PLAYER_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.5,
    a: 1.0,
};
pub const WAYPOINT_COLOR: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
pub const WAYPOINT_LABEL_COLOR: Color = Color {
    r: 0.0,
    g: 0.2,
    b: 0.0,
    a: 1.0,
};

struct MainState {
    player: Actor,
    // font: Font,
    circle_mesh: Mesh,
    last_instant: Instant,
    delta_t: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = Font::new(ctx, "/font.ttf", 12)?;
        let circle_mesh = Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 14.0, 0.4)?;

        let waypoints = vec![
            Waypoint {
                position: Vector2::new(120.0, 120.0),
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
            last_instant: Instant::now(),
            delta_t: 0.0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        self.delta_t = duration_to_f32(self.last_instant.elapsed());
        self.last_instant = Instant::now();

        update_player(self)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // graphics::set_color(ctx, (0, 0, 255).into())?;
        draw_waypoints(ctx, self)?;
        draw_player(ctx, self)?;

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

struct Actor {
    position: Vector2<f32>,
    speed: f32,
    waypoints: Vec<Waypoint>,
    destination_waypoint: usize,
}

struct Waypoint {
    position: Vector2<f32>,
    label: Text,
}

fn draw_waypoints(ctx: &mut Context, state: &MainState) -> GameResult<()> {
    for waypoint in &state.player.waypoints {
        draw_waypoint(ctx, &state.circle_mesh, waypoint)?;
    }
    Ok(())
}

fn draw_player(ctx: &mut Context, state: &MainState) -> GameResult<()> {
    graphics::set_color(ctx, PLAYER_COLOR)?;
    graphics::draw(ctx, &state.circle_mesh, Point2::from_coordinates(state.player.position), 0.0)
}

fn draw_waypoint(ctx: &mut Context, mesh: &Mesh, waypoint: &Waypoint) -> GameResult<()> {
    let offset_x = waypoint.position.x - 5.0;
    let offset_y = waypoint.position.y - 11.0;
    let offset_position = Point2::new(offset_x, offset_y);
    graphics::set_color(ctx, WAYPOINT_COLOR)?;
    graphics::draw(ctx, mesh, Point2::from_coordinates(waypoint.position), 0.0)?;
    graphics::set_color(ctx, WAYPOINT_LABEL_COLOR)?;
    graphics::draw(ctx, &waypoint.label, offset_position, 0.0)?;
    Ok(())
}

fn update_player(state: &mut MainState) -> GameResult<()> {
    move_towards_destination(&mut state.player, &state.delta_t);
    check_if_player_reached_waypoint(&mut state.player);
    Ok(())
}

fn duration_to_f32(duration: Duration)-> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9
}

fn move_towards_destination(actor: &mut Actor, delta_t: &f32) {
    let velocity = actor.speed * delta_t;
    let vector_to_destination = normalize(&(actor.waypoints[actor.destination_waypoint].position - actor.position));
    actor.position = actor.position + (vector_to_destination * velocity);
}

fn check_if_player_reached_waypoint(actor: &mut Actor) {
    let distance = distance(&Point2::from_coordinates(actor.waypoints[actor.destination_waypoint].position), &Point2::from_coordinates(actor.position));
    if distance < 1.0 {
        actor.destination_waypoint = (actor.destination_waypoint + 1) % actor.waypoints.len();
    }
}
