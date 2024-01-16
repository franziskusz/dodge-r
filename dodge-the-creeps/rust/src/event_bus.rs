use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct EventBus {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl EventBus {
    #[signal]
    fn despawned();
}
