use crate::hud;
use crate::main_scene;
use godot::engine::{performance::Monitor, Performance, ProjectSettings};
use godot::prelude::*;
use std::time::SystemTime;
use std::{error::Error, fs, fs::OpenOptions, path::Path, process, str::FromStr};

use csv;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    performance: Gd<Performance>,
    project_setting: Gd<ProjectSettings>,
    second: i64,
    mobs_spawned: i64,
    hits: i64,
    fps: i32,
    memory_static: f64,
    timestamp_micros: u128,
    file_path: String,

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
        let memory_monitor: Monitor = Monitor::MEMORY_STATIC; //TODO remove memory tracking alltogether since its beeing tracked in the system-monitor
        self.memory_static = self.performance.get_monitor(memory_monitor);
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        self.timestamp_micros = duration_since_epoch.as_micros();

        godot_print!(
            "ts, second, mobs, hits, fps, memory {},{},{},{},{},{}",
            self.timestamp_micros.to_string(),
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
        let path = self.file_path.as_str();
        let path_gstring = GString::from_str(path).unwrap();
        let path_globalised = self
            .project_setting
            .globalize_path(path_gstring)
            .to_string();

        //godot_print!("{}", path_globalised); debug

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true) //remove this option if file is ought to be truncated every run
            .open(path_globalised.clone())
            .unwrap();

        let file_size = fs::metadata(path_globalised)
            .expect("file metadata not found")
            .len();

        let mut writer = csv::Writer::from_writer(file);

        if file_size == 0 {
            let header = &["timestamp", "second", "mobs_spawned", "hits", "fps"];
            writer.write_record(header)?;
        }

        let args = &[
            self.timestamp_micros.to_string(),
            self.second.to_string(),
            self.mobs_spawned.to_string(),
            self.hits.to_string(),
            self.fps.to_string(),
        ];

        writer.write_record(args)?;

        writer.flush()?;
        Ok(())
        //write local vars to csv file
    }

    fn make_stats_dir_if_not_exists(&mut self) -> Result<(), Box<dyn Error>> {
        let path = "user://stats/";
        let path_gstring = GString::from_str(path).unwrap();
        let path_globalised = &self
            .project_setting
            .globalize_path(path_gstring)
            .to_string();
        if !Path::new(path_globalised).exists() {
            fs::create_dir(path_globalised)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    #[func]
    fn create_file_path_for_current_run(&mut self) {
        let mut path: String = "user://stats/godot-rust-".to_owned();
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_secs: &str = &*duration_since_epoch.as_secs().to_string();
        let suffix: &str = ".csv";

        path.push_str(timestamp_secs);
        path.push_str(suffix);
        self.file_path = path;
    }
}

#[godot_api]
impl INode for Stats {
    fn init(base: Base<Node>) -> Self {
        Stats {
            performance: Performance::singleton(),
            project_setting: ProjectSettings::singleton(),
            second: 0,
            mobs_spawned: 0,
            hits: 0,
            fps: 0,
            memory_static: 0.0,
            timestamp_micros: 0,
            file_path: String::from(""),
            base,
        }
    }

    fn ready(&mut self) {
        let mut main_scene = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<main_scene::Main>("Main");
        main_scene.connect("send_stats".into(), self.base().callable("update_stats"));

        let mut hud = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<hud::Hud>("Main/Hud");
        hud.connect(
            "start_game".into(),
            self.base().callable("create_file_path_for_current_run"),
        );

        //create stats dir if not already existent
        if let Err(err) = self.make_stats_dir_if_not_exists() {
            godot_print!("{}", err);
            process::exit(1);
        }
        godot_print!("stats ready");
    }
}
