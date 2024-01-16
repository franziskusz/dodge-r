use godot::engine::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_message(&self, text: GString) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.set_text(text);
        message_label.show();

        let mut timer = self.base().get_node_as::<Timer>("MessageTimer");
        timer.start();
    }

    pub fn show_game_over(&self) {
        self.show_message("Game Over".into());

        let mut timer = self.base().get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout".into(), self.base().callable("show_start_button"));
    }

    #[func]
    fn show_start_button(&mut self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.set_text("Dodge the\nCreeps!".into());
        message_label.show();

        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.show();
    }

    #[func]
    pub fn update_score(&self, score: i64) {
        let mut label = self.base().get_node_as::<Label>("ScoreLabel");
        let mut label_text: String = "seconds survived: ".to_owned();
        let score: &str = &*score.to_string();

        label_text.push_str(score);

        label.set_text(label_text.to_string().into());
    }

    #[func]
    pub fn update_hits(&self, hits: i64) {
        let mut label = self.base().get_node_as::<Label>("HitLabel");
        let mut label_text: String = "hits: ".to_owned();
        let hits: &str = &*hits.to_string();

        label_text.push_str(hits);

        label.set_text(label_text.to_string().into());
    }

    #[func]
    pub fn update_mob_counter_label(&self, mob_counter: i64) {
        let mut label = self.base().get_node_as::<Label>("MobLabel");
        let mut label_text: String = "active mobs: ".to_owned();
        let mob_counter: &str = &*mob_counter.to_string();

        label_text.push_str(mob_counter);

        label.set_text(label_text.to_string().into());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.hide();

        // Note: this works only because `start_game` is a deferred signal.
        // This method keeps a &mut Hud, and start_game calls Main::new_game(), which itself accesses this Hud
        // instance through Gd<Hud>::bind_mut(). It will try creating a 2nd &mut reference, and thus panic.
        // Deferring the signal is one option to work around it.
        self.base_mut().emit_signal("start_game".into(), &[]);
    }

    #[func]
    fn on_message_timer_timeout(&self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.hide()
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }
}
