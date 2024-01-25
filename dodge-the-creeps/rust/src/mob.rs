//use crate::event_bus;

use godot::engine::{AnimatedSprite2D, CanvasItem, IRigidBody2D, RigidBody2D};
use godot::prelude::*;
use rand::seq::SliceRandom;

use rand::Rng as _;
use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,
    pub direction: f32,
    pub velocity: Vector2,
    pub is_aiming: bool,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[signal]
    fn despawned();

    #[func]
    fn on_visibility_screen_exited(&mut self) {
        let screen = self
            .base()
            .get_tree()
            .unwrap() //TODO exception handling
            .get_root()
            .unwrap() //TODO exception handling
            .get_node_as::<CanvasItem>("Main/ColorRect");
        let mouse = screen.get_local_mouse_position();
        self.base_mut().look_at(mouse);

        let direction = mouse.angle_to_point(self.base().get_position()) - PI;
        self.direction = direction;

        let mut rng = rand::thread_rng();
        let range = { rng.gen_range(self.min_speed..self.max_speed) };
        self.velocity = Vector2::new(range, 0.0).rotated(real::from_f32(direction));
        self.is_aiming = true;

        //self.base_mut()
        //    .set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        //let mut direction = self.base().get_rotation() + PI / 2.0;
        //direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        //self.base_mut().set_rotation(direction);

        //self.base_mut().emit_signal("despawned".into(), &[]);

        //self.base_mut().queue_free();

        //let mut event_bus = event_bus::EventBus
        //    .self
        //    .base()
        //    .get_node_as::<Node>("EventBus");
        //let mut event_bus = self.base().get_node_as::<Node>("EventBus");
        //event_bus.emit_signal("despawned".into(), &[]);
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    pub fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity;
    }
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
            direction: 0.0,
            velocity: Vector2::new(0.0, 0.0),
            is_aiming: false,
            base,
        }
    }

    fn ready(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.play();
        let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();

        // TODO use pick_random() once implemented
        let anim_names = anim_names.to_vec();
        let mut rng = rand::thread_rng();
        let animation_name = anim_names.choose(&mut rng).unwrap();

        sprite.set_animation(animation_name.into());

        //let range = { rng.gen_range(self.min_speed..self.max_speed) };

        //self.velocity = Vector2::new(range, 0.0).rotated(real::from_f32(direction));
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.is_aiming {
            let velocity = self.velocity;
            self.base_mut().set_linear_velocity(velocity);
        }
    }
}
