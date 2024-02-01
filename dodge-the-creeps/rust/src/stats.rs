use crate::main_scene;
use godot::engine::{performance::Monitor, Performance};
use godot::prelude::*;
use std::time::SystemTime;
use std::{error::Error, fs::OpenOptions, process};

use csv;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    performance: Gd<Performance>,
    second: i64,
    mobs_spawned: i64,
    hits: i64,
    fps: i32,
    memory_static: f64,
    timestamp_nanos: u128,

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
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        self.timestamp_nanos = duration_since_epoch.as_nanos();

        godot_print!(
            "ts, second, mobs, hits, fps, memory {},{},{},{},{},{}",
            self.timestamp_nanos.to_string(),
            self.second.to_string(),
            self.mobs_spawned.to_string(),
            self.hits.to_string(),
            self.fps.to_string(),
            self.memory_static.to_string()
        ); //debug

        // call write_to_csv()
        if let Err(err) = self.write_to_csv() {
            godot_print!("{}", err);
            process::exit(1);
        }
    }

    fn write_to_csv(&mut self) -> Result<(), Box<dyn Error>> {
        //let project_setting: Gd<ProjectSettings> =

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("stats/stats.csv")
            .unwrap();
        let mut writer = csv::Writer::from_writer(file);

        let args = &[
            self.timestamp_nanos.to_string(),
            self.second.to_string(),
            self.mobs_spawned.to_string(),
            self.hits.to_string(),
            self.fps.to_string(),
            self.memory_static.to_string(),
        ];

        writer.write_record(args)?;

        writer.flush()?;
        Ok(())
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
            timestamp_nanos: 0,
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
