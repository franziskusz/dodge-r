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
        //let screen = self
        //    .base()
        //    .get_tree()
        //    .unwrap() //TODO exception handling
        //    .get_root()
        //    .unwrap() //TODO exception handling
        //    .get_node_as::<CanvasItem>("Main/ColorRect");
        //let mouse = screen.get_local_mouse_position();
        //self.base_mut().look_at(mouse);

        self.aim_at_player();

        //self.base_mut().emit_signal("despawned".into(), &[]);

        //self.base_mut().queue_free();
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    pub fn update_target(&mut self, player_position: Vector2) {
        self.target = player_position;
        //debug WARNING: this will overload the output, still can be helpful
        //let target_string: String = self.target.to_string();
        //godot_print!("target: {}", target_string);
    }

    #[func]
    fn aim_at_player(&mut self) {
        let target = self.target;

        self.base_mut().look_at(target);

        let direction = self.base().get_position().angle_to_point(self.target);

        //let direction = mouse.angle_to_point(self.base().get_position()) - PI;
        //self.direction = direction2;

        let mut rng = rand::thread_rng();
        let range = { rng.gen_range(self.min_speed..self.max_speed) };
        self.velocity = Vector2::new(range, 0.0).rotated(real::from_f32(direction));
        self.is_aiming = true;
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

        let target = player.get_position();
        self.target = target;

        self.aim_at_player();
    }

    fn physics_process(&mut self, _delta: f64) {}

    fn integrate_forces(&mut self, mut _physics_state: Gd<PhysicsDirectBodyState2D>) {
        let velocity = self.velocity;
        self.base_mut().set_linear_velocity(velocity);
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
