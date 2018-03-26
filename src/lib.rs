extern crate ggez;
extern crate nalgebra;

use ggez::{Context, GameResult};

use ggez::graphics;
use ggez::graphics::{Color, Font, Mesh, Point2, Text};

use nalgebra::{distance, normalize, Vector2};

pub mod game_timer;

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

pub struct Actor {
    pub position: Vector2<f32>,
    pub speed: f64,
    pub waypoints: Vec<Waypoint>,
}

pub fn draw_player(ctx: &mut Context, player: &Actor, circle_mesh: &Mesh) -> GameResult<()> {
    graphics::set_color(ctx, PLAYER_COLOR)?;
    graphics::draw(
        ctx,
        circle_mesh,
        Point2::from_coordinates(player.position),
        0.0,
    )
}

pub struct Waypoint {
    pub position: Vector2<f32>,
}

impl Waypoint {
    pub fn new(x: f32, y: f32) -> Waypoint {
        Waypoint {
            position: Vector2::new(x, y),
        }
    }
}

pub fn draw_waypoint(ctx: &mut Context, mesh: &Mesh, waypoint: &Waypoint) -> GameResult<()> {
    graphics::set_color(ctx, WAYPOINT_COLOR)?;
    graphics::draw(ctx, mesh, Point2::from_coordinates(waypoint.position), 0.0)
}

pub fn draw_waypoint_labels(
    ctx: &mut Context,
    font: &Font,
    waypoints: &[Waypoint],
) -> GameResult<()> {
    for (index, waypoint) in waypoints.iter().enumerate() {
        let label = Text::new(ctx, &(index + 1).to_string(), font)?;
        let offset_x = waypoint.position.x - 5.0;
        let offset_y = waypoint.position.y - 11.0;
        let offset_position = Point2::new(offset_x, offset_y);
        graphics::set_color(ctx, WAYPOINT_LABEL_COLOR)?;
        graphics::draw(ctx, &label, offset_position, 0.0)?;
    }
    Ok(())
}

pub fn draw_waypoints(
    ctx: &mut Context,
    waypoints: &[Waypoint],
    circle_mesh: &Mesh,
) -> GameResult<()> {
    for waypoint in waypoints {
        draw_waypoint(ctx, circle_mesh, waypoint)?;
    }
    Ok(())
}

pub fn actor_at_waypoint(actor: &Actor) -> bool {
    if !actor.waypoints.is_empty() {
        let distance = distance(
            &Point2::from_coordinates(actor.waypoints[0].position),
            &Point2::from_coordinates(actor.position),
        ) as f64;
        if distance < (actor.speed * 0.01) {
            return true;
        }
    }
    false
}

pub fn move_towards_next_waypoint(actor: &mut Actor, delta_t: &f64) {
    if !actor.waypoints.is_empty() {
        let velocity = (actor.speed * delta_t) as f32;
        let vector_to_destination = normalize(&(actor.waypoints[0].position - actor.position));
        actor.position += vector_to_destination * velocity;
    }
}
