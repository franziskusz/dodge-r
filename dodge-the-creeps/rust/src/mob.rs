//use crate::main_scene;
use crate::player;

use godot::engine::{
    AnimatedSprite2D, IRigidBody2D, InputEvent, PhysicsDirectBodyState2D, RigidBody2D,
};
use godot::prelude::*;
use rand::seq::SliceRandom;

use rand::Rng as _;
//use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,
    pub direction: f32,
    pub velocity: Vector2,
    pub is_aiming: bool,
    pub target: Vector2,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[signal]
    fn despawned();

    #[func]
    fn on_visibility_screen_exited(&mut self) {
        //self.base_mut().set_constant_force(Vector2::new(0.0, 0.0)); //

        //let screen = self
        //    .base()
        //    .get_tree()
        //    .unwrap() //TODO exception handling
        //    .get_root()
        //    .unwrap() //TODO exception handling
        //    .get_node_as::<CanvasItem>("Main/ColorRect");
        //let mouse = screen.get_local_mouse_position();
        //self.base_mut().look_at(mouse);
        let target = self.target;

        self.base_mut().look_at(target);

        let direction = self.base().get_position().angle_to_point(self.target);

        //let direction = mouse.angle_to_point(self.base().get_position()) - PI;
        //self.direction = direction2;

        let mut rng = rand::thread_rng();
        let range = { rng.gen_range(self.min_speed..self.max_speed) };
        self.velocity = Vector2::new(range, 0.0).rotated(real::from_f32(direction));
        //self.base_mut().add_constant_central_force(
        //    Vector2::new(range, 0.0).rotated(real::from_f32(direction)),
        //);
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

    #[func]
    pub fn update_target(&mut self, player_position: Vector2) {
        self.target = player_position;
        //debug WARNING: this will overload the output, still can be helpful
        //let target_string: String = self.target.to_string();
        //godot_print!("target: {}", target_string);
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
            target: Vector2::new(0.0, 0.0),
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
        self.velocity = self.base().get_linear_velocity();

        //let mut main = self
        //    .base()
        //    .get_tree()
        //    .unwrap()
        //    .get_root()
        //    .unwrap()
        //    .get_node_as::<main_scene::Main>("Main");

        //main.connect(
        //    "send_player_position".into(),
        //    self.base().callable("update_target"),
        //);

        let mut player = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<player::Player>("Main/Player");
        player.connect(
            "send_player_position".into(),
            self.base().callable("update_target"),
        );

        //self.connect("send_player_position".into(), )
        //mob.connect("despawned".into(), self.base().callable("on_mob_despawn"));
    }

    fn physics_process(&mut self, _delta: f64) {
        //let player_position = self
        //    .base()
        //    .get_tree()
        //    .unwrap()
        //    .get_root()
        //    .unwrap()
        //    .get_node_as::<main_scene::Main>("Main")
        //    .player_position;
        //self.target = player_position;
    }

    fn integrate_forces(&mut self, mut _physics_state: Gd<PhysicsDirectBodyState2D>) {
        if self.is_aiming {
            let velocity = self.velocity;
            self.base_mut().set_linear_velocity(velocity);
        }
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        //if event.is_action_pressed("click".into()) {
        //let screen = self
        //    .base()
        //    .get_tree()
        //    .unwrap() //TODO exception handling
        //    .get_root()
        //    .unwrap() //TODO exception handling
        //    .get_node_as::<CanvasItem>("Main/ColorRect");
        //self.target = screen.get_local_mouse_position();
        //    let target_string: String = self.target.to_string();
        //    godot_print!("target: {}", target_string);
        //}
    }
}
