use crate::main_scene;
use godot::prelude::*;

//use csv::Writer as _;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    second: i32,
    mobs_spawned: i32,
    fps: i32,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Stats {
    #[func]
    fn update_stats(&mut self, second: i32, mobs_spawned: i64, fps: f64) {
        self.second = second as i32;
        self.mobs_spawned = mobs_spawned as i32;
        self.fps = fps as i32;

        godot_print!(
            "second, mobs, fps {},{},{}",
            self.second.to_string(),
            self.mobs_spawned.to_string(),
            self.fps.to_string()
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
            second: 0,
            mobs_spawned: 0,
            fps: 0,
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
