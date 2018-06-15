extern crate cgmath;
extern crate ggez;
extern crate nalgebra;

use std::f32::consts::PI;

use ggez::{Context, GameResult};

use ggez::graphics;
use ggez::graphics::{Color, Font, Mesh, Point2, Text};

use nalgebra::normalize;

use cgmath::{num_traits::{abs, signum, Num},
             prelude::*,
             Angle,
             Rad,
             Vector2};

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
    pub position: nalgebra::Vector2<f32>,
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
    pub position: nalgebra::Vector2<f32>,
}

impl Waypoint {
    pub fn new(x: f32, y: f32) -> Waypoint {
        Waypoint {
            position: nalgebra::Vector2::new(x, y),
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
        let distance = nalgebra::distance(
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

pub fn limit_vector2(limit: f64, vector: Vector2<f64>) -> Vector2<f64> {
    let mut result = vector;
    if abs(vector.x) > limit {
        result.x = signum(result.x) * limit
    }
    if abs(vector.y) > limit {
        result.y = signum(result.y) * limit
    }

    result
}

pub fn bearing_to_target(origin: cgmath::Point2<f64>, target: cgmath::Point2<f64>) -> f32 {
    let vector: Vector2<f64> = target - origin;
    let rad: Rad<f64> = Angle::atan2(vector.y, vector.x);
    rad.0 as f32 + PI / 2.0
}

pub fn affine_transform<T>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T
where
    T: Num + Copy,
{
    (value - from_min) * ((to_max - to_min) / (from_max - from_min)) + to_min
}

pub fn rotate_vector2(vector2: &mut Vector2<f64>, angle: Rad<f64>) {
    let magnitude: f64 = vector2.magnitude();
    vector2.x = Angle::cos(angle) * magnitude;
    vector2.y = Angle::sin(angle) * magnitude;
}
