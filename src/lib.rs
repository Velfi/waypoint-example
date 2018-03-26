extern crate ggez;
extern crate nalgebra;

use ggez::{Context, GameResult};

use ggez::graphics;
use ggez::graphics::{Color, Mesh, Point2, Text};

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
    pub destination_waypoint: usize,
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
    pub label: Text,
}

pub fn draw_waypoint(ctx: &mut Context, mesh: &Mesh, waypoint: &Waypoint) -> GameResult<()> {
    let offset_x = waypoint.position.x - 5.0;
    let offset_y = waypoint.position.y - 11.0;
    let offset_position = Point2::new(offset_x, offset_y);
    graphics::set_color(ctx, WAYPOINT_COLOR)?;
    graphics::draw(ctx, mesh, Point2::from_coordinates(waypoint.position), 0.0)?;
    graphics::set_color(ctx, WAYPOINT_LABEL_COLOR)?;
    graphics::draw(ctx, &waypoint.label, offset_position, 0.0)?;
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

pub fn move_towards_destination(actor: &mut Actor, delta_t: &f64) {
    let velocity = (actor.speed * delta_t) as f32;
    let vector_to_destination =
        normalize(&(actor.waypoints[actor.destination_waypoint].position - actor.position));
    actor.position += vector_to_destination * velocity;
}

pub fn check_if_player_reached_waypoint(actor: &mut Actor) {
    let distance = distance(
        &Point2::from_coordinates(actor.waypoints[actor.destination_waypoint].position),
        &Point2::from_coordinates(actor.position),
    );
    if distance < 1.0 {
        actor.destination_waypoint = (actor.destination_waypoint + 1) % actor.waypoints.len();
    }
}

pub fn update_player(player: &mut Actor, frame_time: &f64) -> GameResult<()> {
    move_towards_destination(player, frame_time);
    check_if_player_reached_waypoint(player);
    Ok(())
}
