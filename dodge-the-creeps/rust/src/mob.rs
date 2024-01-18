//use crate::event_bus;

use godot::engine::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};
use godot::prelude::*;
use rand::seq::SliceRandom;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[signal]
    fn despawned();

    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.base_mut().emit_signal("despawned".into(), &[]);

        self.base_mut().queue_free();

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
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
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
    }
}
