use godot::engine::{Button, CanvasLayer, CheckButton, ICanvasLayer, Label, Slider, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    is_safe: bool,
    is_bot_player: bool,
    has_weight: bool,
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[signal]
    fn stop_game();

    #[signal]
    fn weight_switch(has_weight: bool);

    #[signal]
    fn safe_mode_switch(is_safe: bool);

    #[signal]
    fn bot_player_switch(is_bot_player: bool);

    #[signal]
    fn mob_spawn_rate(mob_spawn_rate: i64);

    #[func]
    pub fn show_message(&self, text: GString) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.set_text(text);
        message_label.show();

        let mut timer = self.base().get_node_as::<Timer>("MessageTimer");
        timer.start();
    }

    #[func]
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
    pub fn update_fps(&self, fps: f64) {
        let mut label = self.base().get_node_as::<Label>("FramesLabel");
        let mut label_text: String = "fps: ".to_owned();
        let fps: &str = &*fps.to_string();

        label_text.push_str(fps);

        label.set_text(label_text.to_string().into());
        //let fps: godot::engine::Engine.get_frames_per_second(); //only works in debug mode
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut start_button = self.base().get_node_as::<Button>("StartButton");
        start_button.hide();
        let mut stop_button = self.base().get_node_as::<Button>("StopButton");
        stop_button.show();
        let mut safe_mode_switch = self.base().get_node_as::<Button>("SafeModeSwitch");
        safe_mode_switch.hide();
        let mut weight_switch = self.base().get_node_as::<Button>("WeightSwitch");
        weight_switch.hide();
        let mut weight_slider = self.base().get_node_as::<Slider>("WeightSlider");
        weight_slider.hide();
        let mut bot_player_switch = self.base().get_node_as::<Button>("BotPlayerSwitch");
        bot_player_switch.hide();
        let mut initial_wave_slider = self.base().get_node_as::<Slider>("InitialWaveSlider");
        initial_wave_slider.hide();

        // Note: this works only because `start_game` is a deferred signal.
        // This method keeps a &mut Hud, and start_game calls Main::new_game(), which itself accesses this Hud
        // instance through Gd<Hud>::bind_mut(). It will try creating a 2nd &mut reference, and thus panic.
        // Deferring the signal is one option to work around it.
        self.base_mut().emit_signal("start_game".into(), &[]);
    }

    #[func]
    fn on_stop_button_pressed(&mut self) {
        let mut stop_button = self.base().get_node_as::<Button>("StopButton");
        stop_button.hide();
        let mut start_button = self.base().get_node_as::<Button>("StartButton");
        start_button.show();
        let mut safe_mode_switch = self.base().get_node_as::<Button>("SafeModeSwitch");
        safe_mode_switch.show();
        let mut weight_switch = self.base().get_node_as::<Button>("WeightSwitch");
        weight_switch.show();
        if self.has_weight == true {
            let mut weight_slider = self.base().get_node_as::<Slider>("WeightSlider");
            weight_slider.show();
        }
        let mut bot_player_switch = self.base().get_node_as::<Button>("BotPlayerSwitch");
        bot_player_switch.show();
        let mut initial_wave_slider = self.base().get_node_as::<Slider>("InitialWaveSlider");
        initial_wave_slider.show();

        // Note: this works only because `start_game` is a deferred signal.
        // This method keeps a &mut Hud, and start_game calls Main::new_game(), which itself accesses this Hud
        // instance through Gd<Hud>::bind_mut(). It will try creating a 2nd &mut reference, and thus panic.
        // Deferring the signal is one option to work around it.
        self.base_mut().emit_signal("stop_game".into(), &[]);
    }

    #[func]
    fn on_message_timer_timeout(&self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.hide()
    }

    #[func]
    fn init_weight_switch(&mut self) {
        godot_print!("init weight switch"); //debug
        let mut weight_switch = self.base().get_node_as::<CheckButton>("WeightSwitch");
        weight_switch.connect("pressed".into(), self.base().callable("on_weight_switch"));
    }

    #[func]
    fn on_weight_switch(&mut self) {
        self.has_weight = !self.has_weight;

        let args = &[self.has_weight.to_variant()];
        self.base_mut().emit_signal("weight_switch".into(), args);

        let mut button = self.base().get_node_as::<CheckButton>("WeightSwitch");
        let mut button_text: String = "add calculations ".to_owned();
        let mode: &str = &*self.has_weight.to_string();
        button_text.push_str(mode);
        button.set_text(button_text.to_string().into());

        let mut weight_slider = self.base().get_node_as::<Slider>("WeightSlider");
        if self.has_weight == true {
            weight_slider.show();
        } else {
            weight_slider.hide();
        }
    }

    #[func]
    fn on_safe_mode_switch(&mut self) {
        self.is_safe = !self.is_safe;

        let args = &[self.is_safe.to_variant()];
        self.base_mut().emit_signal("safe_mode_switch".into(), args);

        let mut button = self.base().get_node_as::<CheckButton>("SafeModeSwitch");
        let mut button_text: String = "safe mode: ".to_owned();
        let mode: &str = &*self.is_safe.to_string();
        button_text.push_str(mode);
        button.set_text(button_text.to_string().into());
    }

    #[func]
    fn init_bot_player_switch(&mut self) {
        godot_print!("init bot player switch");
        let mut button = self.base().get_node_as::<CheckButton>("BotPlayerSwitch");
        button.connect(
            "pressed".into(),
            self.base().callable("on_bot_player_switch"),
        );
    }

    #[func]
    fn on_bot_player_switch(&mut self) {
        self.is_bot_player = !self.is_bot_player;

        let args = &[self.is_bot_player.to_variant()];
        self.base_mut()
            .emit_signal("bot_player_switch".into(), args);

        let mut button = self.base().get_node_as::<CheckButton>("BotPlayerSwitch");
        let mut button_text: String = "bot player: ".to_owned();
        let mode: &str = &*self.is_bot_player.to_string();
        button_text.push_str(mode);
        button.set_text(button_text.to_string().into());
    }

    #[func]
    fn init_mob_spawn_slider(&mut self) {
        godot_print!("init mob spawn slider"); //debug
        let mut mob_spawn_slider = self.base().get_node_as::<Slider>("MobSpawnSlider");
        mob_spawn_slider.set_use_rounded_values(true);
        mob_spawn_slider.set_min(0.0);
        mob_spawn_slider.set_max(100.0);
        mob_spawn_slider.set_ticks_on_borders(true);

        mob_spawn_slider.connect(
            "value_changed".into(),
            self.base().callable("update_mob_spawn_label"),
        );
        self.update_mob_spawn_label(1.0);
    }

    #[func]
    fn update_mob_spawn_label(&mut self, slider_value: f64) {
        let mob_spawns = slider_value as i64;

        let mut label = self
            .base()
            .get_node_as::<Label>("MobSpawnSlider/SliderLabel");
        let mob_spawns_str: &str = &*mob_spawns.to_string();
        label.set_text(mob_spawns_str.into());

        let mut slider = self.base().get_node_as::<Slider>("MobSpawnSlider");
        slider.release_focus();
    }

    #[func]
    fn init_spawn_intervall_slider(&mut self) {
        godot_print!("init spawn intervall slider"); //debug
        let mut spawn_intervall_slider = self.base().get_node_as::<Slider>("SpawnIntervallSlider");
        spawn_intervall_slider.set_use_rounded_values(true);
        spawn_intervall_slider.set_min(1.0);
        spawn_intervall_slider.set_max(60.0);
        spawn_intervall_slider.set_ticks_on_borders(true);

        spawn_intervall_slider.connect(
            "value_changed".into(),
            self.base().callable("update_spawn_intervall_number_label"),
        );
        self.update_spawn_intervall_number_label(1.0);
    }

    #[func]
    fn update_spawn_intervall_number_label(&mut self, slider_value: f64) {
        let spawn_intervall_length = slider_value as i64;

        let mut label = self
            .base()
            .get_node_as::<Label>("SpawnIntervallSlider/SliderNumberLabel");
        let spawn_intervall_str: &str = &*spawn_intervall_length.to_string();
        label.set_text(spawn_intervall_str.into());

        let mut slider = self.base().get_node_as::<Slider>("SpawnIntervallSlider");
        slider.release_focus();
    }

    #[func]
    fn init_initial_wave_slider(&mut self) {
        godot_print!("init initial wave slider"); //debug
        let mut initial_wave_slider = self.base().get_node_as::<Slider>("InitialWaveSlider");
        initial_wave_slider.set_use_rounded_values(true);
        initial_wave_slider.set_min(0.0);
        initial_wave_slider.set_max(1000.0);
        initial_wave_slider.set_ticks_on_borders(true);

        initial_wave_slider.connect(
            "value_changed".into(),
            self.base().callable("update_initial_wave_number_label"),
        );

        self.update_initial_wave_number_label(0.0);
    }

    #[func]
    fn update_initial_wave_number_label(&mut self, slider_value: f64) {
        let spawn_intervall_length = slider_value as i64;

        let mut label = self
            .base()
            .get_node_as::<Label>("InitialWaveSlider/SliderNumberLabel");
        let initial_wave_str: &str = &*spawn_intervall_length.to_string();
        label.set_text(initial_wave_str.into());

        let mut slider = self.base().get_node_as::<Slider>("InitialWaveSlider");
        slider.release_focus();
    }

    #[func]
    fn init_weight_slider(&mut self) {
        godot_print!("init weight slider"); //debug
        let mut weight_slider = self.base().get_node_as::<Slider>("WeightSlider");
        weight_slider.set_use_rounded_values(true);
        weight_slider.set_min(1.0);
        weight_slider.set_max(1000.0);
        weight_slider.set_ticks_on_borders(true);

        weight_slider.connect(
            "value_changed".into(),
            self.base().callable("update_weight_number_label"),
        );

        self.update_weight_number_label(1.0);
        weight_slider.hide();
    }

    #[func]
    fn update_weight_number_label(&mut self, slider_value: f64) {
        let weight = slider_value as i64;

        let mut label = self
            .base()
            .get_node_as::<Label>("WeightSlider/SliderNumberLabel");
        let weight_str: &str = &*weight.to_string();
        label.set_text(weight_str.into());

        let mut slider = self.base().get_node_as::<Slider>("WeightSlider");
        slider.release_focus();
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<Self::Base>) -> Self {
        Hud {
            has_weight: false,
            is_safe: true,
            is_bot_player: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.init_mob_spawn_slider();
        self.init_spawn_intervall_slider();
        self.init_bot_player_switch();
        self.init_initial_wave_slider();
        self.init_weight_switch();
        self.init_weight_slider();
    }
}
