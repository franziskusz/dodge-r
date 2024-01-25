//use crate::event_bus;
use crate::hud::Hud;
use crate::mob;
use crate::player;

use godot::engine::{Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;

use rand::Rng as _;
use std::f32::consts::PI;

// Deriving GodotClass makes the class available to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: Gd<PackedScene>,
    music: Option<Gd<AudioStreamPlayer>>,
    death_sound: Option<Gd<AudioStreamPlayer>>,
    score: i64,
    hits: i64,
    mob_counter: i64,
    frames: i64,
    fps: f64,
    is_safe: bool,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[signal]
    fn safe_mode_shutdown();

    //#[signal]
    //fn mob_spawned();

    #[func]
    fn game_over(&mut self) {
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        let mut fps_timer = self.base().get_node_as::<Timer>("FPSTimer");

        score_timer.stop();
        mob_timer.stop();
        fps_timer.stop();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().show_game_over();

        self.music().stop();
        self.death_sound().play();
    }

    #[func]
    pub fn new_game(&mut self) {
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");
        let mut player = self.base().get_node_as::<player::Player>("Player");
        let mut start_timer = self.base().get_node_as::<Timer>("StartTimer");

        //godot_singleton().get_frames_per_second();

        self.score = 0;
        self.hits = 0;
        self.mob_counter = 0;
        self.frames = 0;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        let hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.update_hits(self.hits);
        hud.update_mob_counter_label(self.mob_counter);

        player.bind_mut().start(start_position.get_position());
        start_timer.start();

        hud.show_message("Get Ready".into());

        self.music().play();
    }

    #[func]
    fn on_start_timer_timeout(&self) {
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut fps_timer = self.base().get_node_as::<Timer>("FPSTimer");
        mob_timer.start();
        score_timer.start();
        fps_timer.set_wait_time(1.0);
        fps_timer.start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_score(self.score);
    }

    #[func]
    fn on_fps_timer_timeout(&mut self) {
        let frames = self.frames as f64;

        self.fps = frames;
        self.frames = 0;
        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_fps(self.fps);
        let fps_string: String = self.fps.to_string();
        let mob_counter_string: String = self.mob_counter.to_string();
        godot_print!("mobs, fps: {},{}", mob_counter_string, fps_string);
        //TODO: if fps < 30 call game over
        if self.is_safe {
            if self.fps < 20.0 {
                godot_print!("fps limit");
                self.base_mut()
                    .emit_signal("safe_mode_shutdown".into(), &[]);
                //self.game_over();
            }
        }
    }

    #[func]
    fn on_hit_count(&mut self) {
        self.hits += 1;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_hits(self.hits);
    }

    #[func]
    fn update_mob_counter(&mut self, counter: i64) {
        self.mob_counter += counter;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_mob_counter_label(self.mob_counter);
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut i = 0;

        while i < 1 {
            i = i + 1;
            self.spawn_mob();
        }
    }

    #[func]
    fn spawn_mob(&mut self) {
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(mob_scene.clone().upcast());

        let mut mob = mob_scene.cast::<mob::Mob>();
        let range = {
            // Local scope to bind `mob` user object
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };

        mob.callable("set_velocity");

        //(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.connect("start_game".into(), mob.callable("on_start_game"));

        self.update_mob_counter(1);

        //connect to despawn signal
        //mob.despawned.connect();

        mob.connect("despawned".into(), self.base().callable("on_mob_despawn"));
    }

    #[func]
    fn on_mob_despawn(&mut self) {
        self.update_mob_counter(-1);
        self.spawn_mob();
    }

    #[func]
    fn switch_safe_mode(&mut self) {
        self.is_safe = !self.is_safe;
        godot_print!("safemode: {}", self.is_safe.to_string());
    }

    fn music(&mut self) -> &mut AudioStreamPlayer {
        self.music.as_deref_mut().unwrap()
    }

    fn death_sound(&mut self) -> &mut AudioStreamPlayer {
        self.death_sound.as_deref_mut().unwrap()
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            mob_scene: PackedScene::new_gd(),
            score: 0,
            hits: 0,
            mob_counter: 0,
            frames: 0,
            fps: 0.0,
            is_safe: true,
            base,
            music: None,
            death_sound: None,
        }
    }

    fn ready(&mut self) {
        // Note: this is downcast during load() -- completely type-safe thanks to type inference!
        // If the resource does not exist or has an incompatible type, this panics.
        // There is also try_load() if you want to check whether loading succeeded.
        self.mob_scene = load("res://Mob.tscn");
        self.music = Some(self.base().get_node_as("Music"));
        self.death_sound = Some(self.base().get_node_as("DeathSound"));
        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.connect(
            "safe_mode_switch".into(),
            self.base().callable("switch_safe_mode"),
        );
    }

    fn process(&mut self, _delta: f64) {
        self.frames += 1;
    }
}
