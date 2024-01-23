use crate::hud::Hud;
use crate::main_scene;
use godot::prelude::*;

use godot::engine::Timer;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Benchmark {
    frames: i64,
    fps: f64,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Benchmark {
    #[func]
    pub fn signal_received(&mut self) {
        godot_print!("main scene signal received");
    }

    #[func]
    pub fn game_over(&mut self) {
        let mut fps_timer = self.base().get_node_as::<Timer>("FPSTimer");

        fps_timer.stop();
    }

    #[func]
    pub fn new_game(&mut self) {
        self.frames = 0;
    }

    #[func]
    pub fn on_start_timer_timeout(&self) {
        let mut fps_timer = self.base().get_node_as::<Timer>("FPSTimer");
        fps_timer.set_wait_time(1.0);
        fps_timer.start();
    }

    #[func]
    fn on_fps_timer_timeout(&mut self) {
        let frames = self.frames as f64;

        godot_print!("fps count");

        self.fps = frames;
        self.frames = 0;
        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_fps(self.fps);
        //TODO: if fps < 30 call game over
    }
}

#[godot_api]
impl INode for Benchmark {
    fn init(base: Base<Node>) -> Self {
        Benchmark {
            frames: 0,
            fps: 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.base()
            .get_parent()
            .unwrap()
            .connect("game_start_signal".into(), self.base().callable("signal_received"));
    }

    fn process(&mut self, delta: f64) {
        self.frames += 1;
    }
}
