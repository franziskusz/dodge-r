use crate::hud;

use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
    input_position: Vector2,
    is_bot: bool,
    bot_direction: Vector2,
    #[export]
    position: Vector2,

    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[signal]
    fn send_player_position(player_position: Vector2);

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        //self.base_mut().hide();
        self.base_mut().emit_signal("hit".into(), &[]);

        //let mut collision_shape = self
        //    .base()
        //    .get_node_as::<CollisionShape2D>("CollisionShape2D");

        //collision_shape.set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.position = pos;
        self.input_position = pos;
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);

        if self.is_bot {
            let mut bot_timer = self.base().get_node_as::<Timer>("BotTimer");
            bot_timer.start();
            self.bot_direction = Vector2::new(-0.4, 0.4);
        }
    }

    #[func]
    pub fn despawn(&mut self) {
        self.base_mut().hide();
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    fn update_bot_mode(&mut self, bot_mode: bool) {
        self.is_bot = bot_mode;
    }

    #[func]
    fn update_bot_direction(&mut self) {
        if self.bot_direction == Vector2::UP {
            self.bot_direction = Vector2::RIGHT
        } else if self.bot_direction == Vector2::RIGHT {
            self.bot_direction = Vector2::DOWN
        } else if self.bot_direction == Vector2::DOWN {
            self.bot_direction = Vector2::LEFT
        } else if self.bot_direction == Vector2::LEFT {
            self.bot_direction = Vector2::UP
        } else {
            self.bot_direction = Vector2::UP
        }
    }

    #[func]
    fn handle_input(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        if self.is_bot {
            velocity += self.bot_direction;
        }

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("move_up".into()) {
            velocity += Vector2::UP;
        }
        //normalize only if length is over 1.0
        if velocity.length() > 1.0 {
            velocity = velocity.normalized() * self.speed;
        } else if velocity.length() > 0.0 {
            velocity = velocity * self.speed;
        }

        if velocity.length() > 0.0 {
            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else if velocity.y < 0.0 {
                animation = "up";

                //animated_sprite.set_flip_v(velocity.y > 0.0)
            } else {
                animation = "down"
            }

            let change = velocity * real::from_f64(delta);
            let position = self.base().get_global_position() + change;
            let position = Vector2::new(
                position.x.clamp(0.0, self.screen_size.x),
                position.y.clamp(0.0, self.screen_size.y),
            );

            self.input_position = position;

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.stop();
        }
    }

    #[func]
    fn move_player(&mut self) {
        let latest_input_position = self.input_position;

        if self.position != latest_input_position {
            self.position = latest_input_position;
            self.base_mut().set_global_position(latest_input_position);

            let args = &[latest_input_position.to_variant()];
            self.base_mut()
                .emit_signal("send_player_position".into(), args);
        }
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            input_position: Vector2::new(0.0, 0.0),
            position: Vector2::new(0.0, 0.0),
            is_bot: false,
            bot_direction: Vector2::new(-0.4, 0.4),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();

        let mut hud = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<hud::Hud>("Main/Hud");
        hud.connect(
            "bot_player_switch".into(),
            self.base().callable("update_bot_mode"),
        );

        hud.connect("stop_game".into(), self.base().callable("despawn"));

        let mut bot_timer = self.base().get_node_as::<Timer>("BotTimer");
        bot_timer.set_wait_time(2.0);
        bot_timer.connect(
            "timeout".into(),
            self.base().callable("update_bot_direction"),
        );
    }

    fn process(&mut self, _delta: f64) {}

    fn physics_process(&mut self, delta: f64) {
        self.handle_input(delta);
        self.move_player();
    }
}
