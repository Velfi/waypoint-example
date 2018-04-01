extern crate ggez;
extern crate cgmath;
extern crate skunkworks;

use ggez::{
    conf,
    event::{
        self,
        MouseState,
    },
    Context,
    GameResult,
    graphics::{
        self,
        // Font,
        Image,
        Point2
    }
};

use cgmath::{
    Angle,
    Rad,
    Vector2,
    num_traits::{
        Num,
        signum,
        abs,
    },
    prelude::*,
};

use std::f32::consts::PI;

use skunkworks::game_timer::GameTimer;

pub struct MainState {
    // circle_sprite: Image,
    boid_sprite: Image,
    game_timer: GameTimer,
    vehicle: Vehicle,
    // font: Font,
    mouse_position: cgmath::Point2<f64>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // let font = Font::new(ctx, "/font.ttf", 12)?;
        // let mut circle_sprite = Image::new(ctx, "/circle.png")?;
        let boid_sprite = Image::new(ctx, "/boid.png")?;

        let vehicle = Vehicle::new(cgmath::Point2::new(400.0, 300.0));

        let s = MainState {
            // font,
            // circle_sprite,
            boid_sprite,
            vehicle,
            mouse_position: cgmath::Point2::new(0.0, 0.0),
            game_timer: GameTimer::new(),
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _mouse_state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
        self.mouse_position.x = f64::from(x);
        self.mouse_position.y = f64::from(y);
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.game_timer.tick();
        // self.vehicle.seek(self.mouse_position);
        self.vehicle.arrive(self.mouse_position);
        self.vehicle.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw_ex(
            ctx,
            &self.boid_sprite,
            graphics::DrawParam {
                dest: Point2::new(self.vehicle.location.x as f32, self.vehicle.location.y as f32),
                rotation: bearing_to_target(self.vehicle.location, self.mouse_position),
                offset: Point2::new(0.5, 0.5),
                ..Default::default()
            },
        )?;

        graphics::present(ctx);

        if (self.game_timer.get_ticks() % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }
}

pub fn main() {
    use std::{env, path};

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Test", "Waypoint", c).unwrap();

    let test_vec = Vector2::new(100.0, 50.0);
    println!("before normalize: {:?}", test_vec);
    let test_vec = test_vec.normalize();
    println!("after normalize: {:?}", test_vec);

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

struct Vehicle {
    location: cgmath::Point2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    maxforce: f64,
    maxspeed: f64,
}

impl Vehicle {
    fn new(location: cgmath::Point2<f64>)-> Vehicle {
        Vehicle {
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            location,
            maxspeed: 5.0,
            maxforce: 0.1,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        limit_vector2(self.maxspeed, &mut self.velocity);
        self.location += self.velocity;
        self.acceleration *= 0.0; // acceleration.mult(0);
    }

    fn apply_force(&mut self, force: Vector2<f64>) {
        self.acceleration += force;
    }

    fn seek(&mut self, target: cgmath::Point2<f64>) {
        let mut desired = target - self.location;
        desired = desired.normalize();
        desired *= self.maxspeed;

        let mut steer = desired - self.velocity;
        limit_vector2(self.maxforce, &mut steer);
        self.apply_force(steer);
    }

    fn arrive(&mut self, target: cgmath::Point2<f64>) {
        let mut desired = target - self.location;
        let d_magnitude = desired.magnitude();
        desired = desired.normalize();

        if d_magnitude < 100.0 {
            let m = affine_transform(d_magnitude, 0.0, 100.0, 0.0, self.maxspeed);
            desired *= m;
        } else {
            desired *= self.maxspeed;
        }

        let mut steer = desired - self.velocity;
        limit_vector2(self.maxforce, &mut steer);
        self.apply_force(steer);
    }
}

fn limit_vector2(limit: f64, vector: &mut Vector2<f64>) {
    if abs(vector.x) > limit {
        vector.x = signum(vector.x) * limit
    }
    if abs(vector.y) > limit {
        vector.y = signum(vector.y) * limit
    }
}

fn bearing_to_target(origin: cgmath::Point2<f64>, target: cgmath::Point2<f64>)-> f32 {
    let vector: Vector2<f64> = target - origin;
    let rad: Rad<f64> = Angle::atan2(vector.y, vector.x);
    rad.0 as f32 + PI / 2.0
}

fn affine_transform<T>(value: T, from_min: T, from_max: T, to_min: T, to_max: T)-> T
    where T: Num + Copy {
    (value - from_min) * ((to_max - to_min) / (from_max - from_min)) + to_min
}