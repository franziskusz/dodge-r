use crate::main_scene;
use godot::engine::{performance::Monitor, Performance};
use godot::prelude::*;

//use csv::Writer as _;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    performance: Gd<Performance>,
    second: i64,
    mobs_spawned: i64,
    hits: i64,
    fps: i32,
    memory_static: f64,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Stats {
    #[func]
    fn update_stats(&mut self, second: i64, mobs_spawned: i64, hits: i64, fps: f64) {
        self.second = second;
        self.mobs_spawned = mobs_spawned;
        self.hits = hits;
        self.fps = fps as i32;
        let memory_monitor: Monitor = Monitor::MEMORY_STATIC;
        self.memory_static = self.performance.get_monitor(memory_monitor);

        godot_print!(
            "second, mobs, hits, fps, memory {},{},{},{},{}",
            self.second.to_string(),
            self.mobs_spawned.to_string(),
            self.hits.to_string(),
            self.fps.to_string(),
            self.memory_static.to_string()
        ); //debug

        // call write_to_csv()
    }

    #[func]
    fn write_to_csv() {
        //write local vars to csv file
    }
}

#[godot_api]
impl INode for Stats {
    fn init(base: Base<Node>) -> Self {
        Stats {
            performance: Performance::singleton(),
            second: 0,
            mobs_spawned: 0,
            hits: 0,
            fps: 0,
            memory_static: 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("stats ready");
        let mut main_scene = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<main_scene::Main>("Main");
        main_scene.connect("send_stats".into(), self.base().callable("update_stats"));
    }
}
