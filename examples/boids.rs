extern crate cgmath;
extern crate ggez;
extern crate rand;
extern crate skunkworks;

use ggez::{conf,
           event::{self, MouseState},
           graphics::{self,
                      // Font,
                      Image,
                      Point2},
           Context,
           GameResult};

use cgmath::{prelude::*, Rad, Vector2};

use rand::{thread_rng, Rng, ThreadRng};

use std::cell::RefCell;

use skunkworks::{affine_transform, bearing_to_target, game_timer::GameTimer, limit_vector2};

const HD: (u32, u32) = (1280, 720);
const FULL_HD: (u32, u32) = (1920, 1080);
const UHD: (u32, u32) = (3840, 2160);

const WXGA: (u32, u32) = (1280, 800);
const WUXGA: (u32, u32) = (1920, 1200);

const BOID_COUNT: u32 = 200;
const RESOLUTION: (u32, u32) = WUXGA;
const MAX_FORCE: f64 = 0.03;
const MAX_SPEED: f64 = 2.0;
const CIRCLE_RADIUS: f64 = 100.0; // Radius of the wandering circle
const RADIAN_DELTA: f64 = 0.017_453_3f64 * 15.0; // Maximum degree of variance when wandering
const WANDER_WEIGHT: f64 = 1.0;
const SEPARATION_RANGE: f64 = 80.0;
const SEPARATION_WEIGHT: f64 = 1.5;
const ALIGN_RANGE: f64 = 160.0;
const ALIGN_WEIGHT: f64 = 1.0;
const COHESION_RANGE: f64 = 160.0;
const COHESION_WEIGHT: f64 = 1.0;
const CONSTRAIN_DISTANCE: f32 = 20.0;

const BG_SCALE: f32 = RESOLUTION.0 as f32 / 1280 as f32;

const ZERO_VECTOR: Vector2<f64> = Vector2 { x: 0f64, y: 0f64 };

pub struct MainState {
    // circle_sprite: Image,
    bg_image: Image,
    boid_image: Image,
    game_timer: GameTimer,
    // font: Font,
    rng_seed: ThreadRng,
    vehicles: Vec<RefCell<Vehicle>>,
    mouse_position: cgmath::Point2<f64>,
    default_offset: Point2,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // let font = Font::new(ctx, "/font.ttf", 12)?;
        let bg_image = Image::new(ctx, "/water.png")?;
        let boid_image = Image::new(ctx, "/koi.png")?;
        let mut rng_seed = thread_rng();
        let mut vehicles = Vec::new();
        for _i in 0..BOID_COUNT {
            vehicles.push(RefCell::new(Vehicle::new(
                Vector2::new(
                    rng_seed.gen_range(-MAX_SPEED, MAX_SPEED),
                    rng_seed.gen_range(-MAX_SPEED, MAX_SPEED),
                ),
                cgmath::Point2::new(
                    rng_seed.gen_range(0f64, f64::from(RESOLUTION.0)),
                    rng_seed.gen_range(0f64, f64::from(RESOLUTION.1)),
                ),
            )));
        }

        println!("Added {} vehicles.", vehicles.len());

        let s = MainState {
            // font,
            bg_image,
            boid_image,
            vehicles,
            rng_seed,
            mouse_position: cgmath::Point2::new(0.0, 0.0),
            game_timer: GameTimer::new(),
            default_offset: Point2::new(0.5, 0.5),
        };
        println!("BG_SCALE: {}", BG_SCALE);

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

        for vehicle in &self.vehicles {
            vehicle
                .borrow_mut()
                .update(&self.vehicles, &mut self.rng_seed, self.mouse_position);
        }
        // _ctx.quit();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw_ex(
            ctx,
            &self.bg_image,
            graphics::DrawParam {
                dest: Point2::new(0.0, 0.0),
                scale: Point2::new(BG_SCALE, BG_SCALE),
                ..Default::default()
            },
        )?;

        for vehicle in &self.vehicles {
            graphics::draw_ex(
                ctx,
                &self.boid_image,
                graphics::DrawParam {
                    dest: Point2::new(
                        vehicle.borrow().location.x as f32,
                        vehicle.borrow().location.y as f32,
                    ),
                    rotation: vehicle.borrow().get_bearing(),
                    offset: self.default_offset,
                    ..Default::default()
                },
            )?;
        }

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
    c.window_mode.width = RESOLUTION.0;
    c.window_mode.height = RESOLUTION.1;
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
    location: cgmath::Point2<f64>,
    wander_angle: Rad<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    max_force: f64,
    max_speed: f64,
}

impl Vehicle {
    fn new(velocity: Vector2<f64>, location: cgmath::Point2<f64>) -> Vehicle {
        Vehicle {
            location,
            wander_angle: Rad(0.0),
            velocity,
            acceleration: Vector2::new(0.0, 0.0),
            max_force: MAX_FORCE,
            max_speed: MAX_SPEED,
        }
    }

    fn update(
        &mut self,
        vehicles: &[RefCell<Vehicle>],
        rng_seed: &mut ThreadRng,
        mouse_position: cgmath::Point2<f64>,
    ) {
        let mut separate = self.separate(vehicles);
        let mut align = self.align(vehicles);
        let mut cohere = self.cohesion(vehicles);
        let mut wander = self.wander(rng_seed.next_f64());

        separate *= SEPARATION_WEIGHT;
        align *= ALIGN_WEIGHT;
        cohere *= COHESION_WEIGHT;
        wander *= WANDER_WEIGHT;

        self.apply_force(separate);
        self.apply_force(align);
        self.apply_force(cohere);
        self.apply_force(wander);

        self.apply_acceleration();

        self.constrain_location(
            -CONSTRAIN_DISTANCE,
            RESOLUTION.0 as f32 + CONSTRAIN_DISTANCE,
            -CONSTRAIN_DISTANCE,
            RESOLUTION.1 as f32 + CONSTRAIN_DISTANCE,
        );
    }

    fn apply_force(&mut self, force: Vector2<f64>) {
        let force = limit_vector2(self.max_force, force);
        self.acceleration += force;
    }

    fn apply_acceleration(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = limit_vector2(self.max_speed, self.velocity);
        self.location += self.velocity;
        self.acceleration *= 0.0;
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

    fn constrain_location(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        if (self.location.x as f32) < x_min {
            self.location.x = f64::from(x_max);
        } else if (self.location.x as f32) > x_max {
            self.location.x = f64::from(x_min);
        }

        if (self.location.y as f32) < y_min {
            self.location.y = f64::from(y_max);
        } else if (self.location.y as f32) > y_max {
            self.location.y = f64::from(y_min);
        }
    }

    fn flee(&mut self, target: cgmath::Point2<f64>) {
        let safety_range = 200.0;
        let mut desired = self.location - target;

        if desired.magnitude() < safety_range {
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

    fn wander(&mut self, rng_f64: f64) -> Vector2<f64> {
        let center = match self.velocity {
            Vector2 { x, y } if x == 0.0 && y == 0.0 => self.location,
            Vector2 { x, y } if x.is_nan() || y.is_nan() => self.location,
            _ => self.location + (self.velocity * CIRCLE_RADIUS),
        };

        self.wander_angle += Rad(rng_f64 * RADIAN_DELTA) - Rad(RADIAN_DELTA * 0.5);

        let x = CIRCLE_RADIUS * Angle::cos(self.wander_angle);
        let y = CIRCLE_RADIUS * Angle::sin(self.wander_angle);
        let offset = Vector2::new(x, y);

        let mut desired = center + offset - self.location;
        let distance = desired.magnitude();

        if distance > 1.0 {
            desired = desired.normalize();
            desired *= self.max_speed;
            desired - self.velocity
        } else {
            ZERO_VECTOR
        }
    }

    fn separate(&mut self, vehicles: &[RefCell<Vehicle>]) -> Vector2<f64> {
        let mut sum = Vector2::new(0f64, 0f64);
        let mut count = 0;

        for vehicle in vehicles {
            if vehicle.as_ptr() != self as *mut Vehicle {
                let neighbour = vehicle.borrow();
                let d = self.location.distance(neighbour.location);
                if d < SEPARATION_RANGE {
                    let mut diff = self.location - neighbour.location;
                    diff = diff.normalize();
                    diff /= d;
                    sum += diff;
                    count += 1;
                }
            }
        }

        if count > 0 {
            sum /= f64::from(count);
            sum = sum.normalize();
            sum *= MAX_SPEED;
            (sum - self.velocity)
        } else {
            ZERO_VECTOR
        }
    }

    fn align(&mut self, vehicles: &[RefCell<Vehicle>]) -> cgmath::Vector2<f64> {
        let mut sum = Vector2::new(0f64, 0f64);
        let mut count = 0;

        for vehicle in vehicles {
            if vehicle.as_ptr() != self as *mut Vehicle {
                let neighbour = vehicle.borrow();
                let d = self.location.distance(neighbour.location);
                if d < ALIGN_RANGE {
                    sum += neighbour.velocity;
                    count += 1;
                }
            }
        }

        if count > 0 {
            sum /= f64::from(count);
            sum = sum.normalize();
            sum *= MAX_SPEED;
            (sum - self.velocity)
        } else {
            ZERO_VECTOR
        }
    }

    fn cohesion(&mut self, vehicles: &[RefCell<Vehicle>]) -> cgmath::Vector2<f64> {
        let mut sum = ZERO_VECTOR.clone();
        let mut count = 0f64;

        for vehicle in vehicles {
            if vehicle.as_ptr() != self as *mut Vehicle {
                let neighbour = vehicle.borrow();
                let d = self.location.distance(neighbour.location);
                if d < COHESION_RANGE {
                    sum += neighbour.location.to_vec();
                    count += 1.0;
                }
            }
        }

        if count > 0.0 {
            (sum / count)
        } else {
            ZERO_VECTOR
        }
    }
}
