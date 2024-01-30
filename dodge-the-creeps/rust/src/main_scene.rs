//use crate::event_bus;
use crate::hud::Hud;
use crate::mob;
use crate::player;

use godot::engine::{Marker2D, PathFollow2D, RigidBody2D, Slider, Timer};
use godot::prelude::*;

use rand::Rng as _;
//use std::f32::consts::PI;
const FPS_LOWER_LIMT: f64 = 20.0;

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
    mob_spawns_per_second: i64,
    spawn_intervall_length: i64,
    wave_size: i64,
    #[base]
    base: Base<Node>,
    #[export]
    player_position: Vector2,
    is_safe: bool,
}

#[godot_api]
impl Main {
    #[signal]
    fn safe_mode_shutdown();

    #[signal]
    fn send_player_position(player_position: Vector2);

    #[signal]
    fn send_stats(second: i32, mobs_spawned: i64, fps: f64);

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

        self.score = 0;
        self.hits = 0;
        self.mob_counter = 0;
        self.frames = 0;

        let initial_wave_size = self.mob_spawns_per_second;
        self.wave_size = initial_wave_size;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        let hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.update_hits(self.hits);
        hud.update_mob_counter_label(self.mob_counter);

        //debug
        //godot_print!(
        //    "A start pos, player pos: {},{}",
        //    start_position.get_position().to_string(),
        //    self.player_position.to_string(),
        //);
        self.player_position = start_position.get_position();
        player.bind_mut().start(start_position.get_position());

        start_timer.start();

        hud.show_message("Get Ready".into());

        self.music().play();
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut fps_timer = self.base().get_node_as::<Timer>("FPSTimer");
        mob_timer.start();
        score_timer.start();
        fps_timer.set_wait_time(1.0);
        fps_timer.start();
        self.frames = 0;

        let initial_wave_size = self.mob_spawns_per_second;
        self.wave_size = initial_wave_size;
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;

        //let args = &[
        //    self.score.to_variant(),
        //    self.mob_counter.to_variant(),
        //    self.fps.to_variant(),
        //];

        //self.base_mut().emit_signal("send_stats".into(), args);

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
        //let fps_string: String = self.fps.to_string();
        //let mob_counter_string: String = self.mob_counter.to_string();
        //godot_print!("mobs, fps: {},{}", mob_counter_string, fps_string); //debug

        let args = &[
            self.score.to_variant(),
            self.mob_counter.to_variant(),
            self.hits.to_variant(),
            self.fps.to_variant(),
        ];

        self.base_mut().emit_signal("send_stats".into(), args);

        if self.is_safe {
            if self.fps < FPS_LOWER_LIMT {
                //TODO make this accessible via main menu
                godot_print!("fps limit");
                self.base_mut()
                    .emit_signal("safe_mode_shutdown".into(), &[]);
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
        let wave_size_float = self.wave_size as f64;
        let spawn_intervall_length_float = self.spawn_intervall_length as f64;
        let mob_spawns_per_second_float = self.mob_spawns_per_second as f64;

        let loop_var = wave_size_float / spawn_intervall_length_float;
        if loop_var < mob_spawns_per_second_float {
            self.wave_size += self.mob_spawns_per_second;
        } else {
            let mut i = 0;
            while i < self.wave_size {
                i = i + 1;

                self.spawn_mob();
            }
            self.wave_size = self.mob_spawns_per_second;
        }

        //let mut j = 0;
        //while j < self.mob_spawns_per_second {
        //    j = j + 1;

        //    self.spawn_mob();
        //}
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

        //godot_print!("C direction: {}", direction.to_string()); //debug
        //godot_print!("C player pos: {}", self.player_position.to_string());//debug

        let player = self.base().get_node_as::<player::Player>("Player");
        let start_target = player.get_position();

        let start_angle = mob_spawn_location
            .get_position()
            .angle_to_point(start_target);

        //godot_print!("C angle to player: {}", start_angle.to_string());//debug
        mob_scene.set_rotation(start_angle);

        self.base_mut().add_child(mob_scene.clone().upcast());

        let mut mob = mob_scene.cast::<mob::Mob>();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.connect("start_game".into(), mob.callable("on_start_game"));

        self.update_mob_counter(1);

        mob.connect("despawned".into(), self.base().callable("on_mob_despawn"));
    }

    #[func]
    fn on_mob_despawn(&mut self) {
        self.update_mob_counter(-1);
        self.spawn_mob();
    }

    #[func]
    fn switch_safe_mode(&mut self, safe_mode: bool) {
        self.is_safe = safe_mode;
        godot_print!("safemode: {}", self.is_safe.to_string());
    }

    #[func]
    pub fn update_player_position(&mut self, player_position: Vector2) {
        self.player_position = player_position;
        //debug WARNING: this will overload the output, still can be helpful
        //let target_string: String = self.target.to_string();
        //godot_print!("target: {}", target_string);
    }

    #[func]
    pub fn update_mob_spawn_rate(&mut self, slider_value: f64) {
        let mob_spawns = slider_value as i64;
        self.mob_spawns_per_second = mob_spawns;
        //godot_print!("mob spawns/s {}", mob_spawns.to_string()); //debug
    }

    #[func]
    pub fn update_spawn_intervall_length(&mut self, slider_value: f64) {
        let intervall_length = slider_value as i64;
        self.spawn_intervall_length = intervall_length;
        //godot_print!("intervall length {}", intervall_length.to_string()); //debug
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
            mob_spawns_per_second: 1,
            spawn_intervall_length: 1,
            wave_size: 0,
            player_position: Vector2::new(0.0, 0.0),
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

        let mut mob_spawn_slider = self.base().get_node_as::<Slider>("Hud/MobSpawnSlider");
        mob_spawn_slider.connect(
            "value_changed".into(),
            self.base().callable("update_mob_spawn_rate"),
        );

        let mut spawn_intervall_slider = self
            .base()
            .get_node_as::<Slider>("Hud/SpawnIntervallSlider");
        spawn_intervall_slider.connect(
            "value_changed".into(),
            self.base().callable("update_spawn_intervall_length"),
        );

        let mut player = self.base().get_node_as::<player::Player>("Player");
        player.connect(
            "send_player_position".into(),
            self.base().callable("update_player_position"),
        );
    }

    fn process(&mut self, _delta: f64) {
        self.frames += 1;
    }

    fn physics_process(&mut self, _delta: f64) {}
}
