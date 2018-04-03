extern crate cgmath;
extern crate ggez;
extern crate rand;
extern crate skunkworks;

use ggez::{conf, Context, GameResult, event::{self, MouseState},
           graphics::{self,
                      // Font,
                      Image,
                      Point2}};

use cgmath::{Rad, Vector2, prelude::*};

use rand::{thread_rng, Rng, ThreadRng};

use skunkworks::{affine_transform, bearing_to_target, game_timer::GameTimer, limit_vector2};

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 1280;
const MAX_FORCE: f64 = 0.8;
const MAX_SPEED: f64 = 4.0;
const CIRCLE_RADIUS: f64 = 100.0; // Radius of the wandering circle
const RADIAN_DELTA: f64 = 0.017_453_3f64 * 20.0; // Maximum degree of variance when wandering

pub struct MainState {
    // circle_sprite: Image,
    boid_sprite: Image,
    game_timer: GameTimer,
    vehicle: Vehicle,
    // font: Font,
    rng_seed: ThreadRng,
    mouse_position: cgmath::Point2<f64>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // let font = Font::new(ctx, "/font.ttf", 12)?;
        // let mut circle_sprite = Image::new(ctx, "/circle.png")?;
        let boid_sprite = Image::new(ctx, "/boid.png")?;
        let vehicle = Vehicle::new(VehicleBehavior::Wander, cgmath::Point2::new(400.0, 300.0));
        let rng_seed = thread_rng();

        let s = MainState {
            // font,
            // circle_sprite,
            boid_sprite,
            vehicle,
            rng_seed,
            mouse_position: cgmath::Point2::new(0.0, 0.0),
            game_timer: GameTimer::new(),
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _mouse_state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        self.mouse_position.x = f64::from(x);
        self.mouse_position.y = f64::from(y);
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.game_timer.tick();

        match self.vehicle.behavior {
            VehicleBehavior::Seek => self.vehicle.seek(self.mouse_position),
            VehicleBehavior::Flee => self.vehicle.flee(self.mouse_position),
            VehicleBehavior::Arrive => self.vehicle.arrive(self.mouse_position),
            VehicleBehavior::Wander => self.vehicle.wander(self.rng_seed.next_f64()),
        }

        self.vehicle.update();
        // _ctx.quit();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw_ex(
            ctx,
            &self.boid_sprite,
            graphics::DrawParam {
                dest: Point2::new(
                    self.vehicle.location.x as f32,
                    self.vehicle.location.y as f32,
                ),
                rotation: self.vehicle.get_bearing(),
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

    let mut c = conf::Conf::new();
    c.window_mode.width = SCREEN_WIDTH;
    c.window_mode.height = SCREEN_HEIGHT;
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

#[derive(Debug)]
struct Vehicle {
    behavior: VehicleBehavior,
    location: cgmath::Point2<f64>,
    wander_angle: Rad<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    max_force: f64,
    max_speed: f64,
}

impl Vehicle {
    fn new(behavior: VehicleBehavior, location: cgmath::Point2<f64>) -> Vehicle {
        Vehicle {
            behavior,
            location,
            wander_angle: Rad(0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            max_force: MAX_FORCE,
            max_speed: MAX_SPEED,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = limit_vector2(self.max_speed, self.velocity);
        self.location += self.velocity;
        self.acceleration *= 0.0;

        if (self.location.x as f32) < 0.0 {
            self.location.x = f64::from(SCREEN_WIDTH);
        } else if (self.location.x as f32) > (SCREEN_WIDTH as f32) {
            self.location.x = 0.0;
        }

        if (self.location.y as f32) < 0.0 {
            self.location.y = f64::from(SCREEN_HEIGHT);
        } else if (self.location.y as f32) > (SCREEN_HEIGHT as f32) {
            self.location.y = 0.0;
        }
    }

    fn apply_force(&mut self, force: Vector2<f64>) {
        let force = limit_vector2(self.max_force, force);
        self.acceleration += force;
    }

    fn get_bearing(&self) -> f32 {
        bearing_to_target(self.location, self.location + self.velocity)
    }

    fn seek(&mut self, target: cgmath::Point2<f64>) {
        let mut desired = target - self.location;
        let distance = desired.magnitude();

        if distance > 1.0 {
            desired = desired.normalize();
            desired *= self.max_speed;
            let steer = desired - self.velocity;
            self.apply_force(steer)
        }
    }

    fn flee(&mut self, target: cgmath::Point2<f64>) {
        let safety_range = 200.0;
        let mut desired = self.location - target;
        let distance = desired.magnitude();

        if distance < safety_range {
            desired = desired.normalize();
            desired *= self.max_speed;
            let steer = desired - self.velocity;
            self.apply_force(steer)
        }
    }

    fn arrive(&mut self, target: cgmath::Point2<f64>) {
        let mut desired = target - self.location;
        let distance = desired.magnitude();
        desired = desired.normalize();

        if distance < 100.0 {
            let m = affine_transform(distance, 0.0, 100.0, 0.0, self.max_speed);
            desired *= m;
        } else {
            desired *= self.max_speed;
        }

        let steer = desired - self.velocity;
        self.apply_force(steer);
    }

    fn wander(&mut self, rng_f64: f64) {
        let center = match self.velocity {
            Vector2 { x, y } if x == 0.0 && y == 0.0 => self.location,
            Vector2 { x, y } if x.is_nan() || y.is_nan() => self.location,
            _ => self.location + (self.velocity * CIRCLE_RADIUS),
        };

        self.wander_angle += Rad(rng_f64 * RADIAN_DELTA) - Rad(RADIAN_DELTA * 0.5);

        let x = CIRCLE_RADIUS * Angle::cos(self.wander_angle);
        let y = CIRCLE_RADIUS * Angle::sin(self.wander_angle);
        let offset = Vector2::new(x, y);
        self.seek(center + offset);
    }
}

#[derive(Debug)]
enum VehicleBehavior {
    Flee,
    Seek,
    Arrive,
    Wander,
}
