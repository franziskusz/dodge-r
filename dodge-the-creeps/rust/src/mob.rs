//use crate::main_scene;
use crate::hud;
use crate::player;

use godot::engine::{
    AnimatedSprite2D, IRigidBody2D, InputEvent, PhysicsDirectBodyState2D, RigidBody2D,
};
use godot::prelude::*;
use rand::seq::SliceRandom;

use rand::Rng as _;
//use std::f32::consts::PI;
const INITIAL_FORCE_DIVISOR: f32 = 35.0;
const AIMING_FORCE_DIVISOR: f32 = 5.0;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,
    pub direction: f32,
    pub speed: f32,
    pub velocity: Vector2,
    //pub is_aiming: bool,
    pub target: Vector2,
    pub aiming_direction: Vector2,  //updatet on moving target
    pub initial_direction: Vector2, //updatet on init and when bouncing off wall
    pub has_weight: bool,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[signal]
    fn despawned();

    #[func]
    pub fn set_weight(&mut self, has_weight: bool) {
        self.has_weight = has_weight;
        //godot_print!("this mob has weight: {}", has_weight)
    }

    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.aim_at_player();

        //let screen = self
        //    .base()
        //    .get_tree()
        //    .unwrap() //TODO exception handling
        //    .get_root()
        //    .unwrap() //TODO exception handling
        //    .get_node_as::<CanvasItem>("Main/ColorRect");
        //let mouse = screen.get_local_mouse_position();
        //self.base_mut().look_at(mouse);

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
        self.update_aiming_direction();
        //debug WARNING: this will overload the output, still can be helpful
        //let target_string: String = self.target.to_string();
        //godot_print!("target: {}", target_string);
    }

    #[func]
    pub fn update_aiming_direction(&mut self) {
        let direction = self.base().get_position().angle_to_point(self.target);

        self.aiming_direction = Vector2::new(self.speed, 0.0).rotated(real::from_f32(direction));
    }

    #[func]
    fn aim_at_player(&mut self) {
        let target = self.target;
        //self.initial_target = target;
        //godot_print!("2. updatetd target {}", target.to_string()); //debug

        self.base_mut().look_at(target);

        let direction = self.base().get_position().angle_to_point(self.target);

        //let direction = mouse.angle_to_point(self.base().get_position()) - PI;
        //self.direction = direction2;

        self.velocity = Vector2::new(self.speed, 0.0).rotated(real::from_f32(direction));
        let velocity = self.velocity;
        self.base_mut().set_linear_velocity(velocity);
        //self.is_aiming = true;

        self.aiming_direction = velocity;
        self.initial_direction = velocity;
    }

    #[func]
    pub fn on_game_over_despawn(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn lift_weight(&mut self) {
        let radius: f32 = 10.0;
        let count: i32 = 500;
        let countf = count as f32;
        //let position = self.base().get_position();
        let direction = self.aiming_direction;
        //let test = self.aiming_direction;

        for n in 0..count {
            let x = f32::sin(n as f32 / countf * 360.0) * radius;
            let y = f32::cos(n as f32 / countf * 360.0) * radius;
            let target = Vector2::new(x, y) * direction / 30.0;
            self.base_mut().draw_line(
                Vector2::new(0.0, 0.0),
                target,
                Color::from_rgba(0.7, 0.2, 0.0, 0.5),
            )
        }
    }
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
            direction: 0.0,
            speed: 0.0,
            velocity: Vector2::new(0.0, 0.0),
            //is_aiming: false,
            target: Vector2::new(0.0, 0.0),
            aiming_direction: Vector2::new(0.0, 0.0),
            initial_direction: Vector2::new(0.0, 0.0),
            has_weight: false,
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

        let mut hud = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<hud::Hud>("Main/Hud");

        hud.connect(
            "stop_game".into(),
            self.base().callable("on_game_over_despawn"),
        );

        let target = player.get_position();
        self.target = target;

        let mut rng = rand::thread_rng();
        self.speed = rng.gen_range(self.min_speed..self.max_speed);

        //godot_print!("1. initial target {}", target.to_string()); //debug

        self.aim_at_player();

        if self.has_weight == true {
            self.base_mut().queue_redraw();
        }
    }

    fn draw(&mut self) {
        if self.has_weight == true {
            self.lift_weight();
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let initial_force_divisor_vec = Vector2::new(INITIAL_FORCE_DIVISOR, INITIAL_FORCE_DIVISOR);
        let aiming_force_divisor_vec = Vector2::new(AIMING_FORCE_DIVISOR, AIMING_FORCE_DIVISOR);
        let aiming_direction = self.aiming_direction / aiming_force_divisor_vec;
        self.base_mut().apply_force(aiming_direction);
        let initial_direction = self.initial_direction / initial_force_divisor_vec;
        self.base_mut().apply_force(initial_direction);
    }

    fn integrate_forces(&mut self, mut _physics_state: Gd<PhysicsDirectBodyState2D>) {
        let target = self.target;
        self.base_mut().look_at(target);

        //let velocity = self.velocity;
        //self.base_mut().set_linear_velocity(velocity);
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
