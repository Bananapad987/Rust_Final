use godot::prelude::*;

#[derive(GodotClass)]
#[class(init)]
pub struct Attack {
    #[var]
    #[init(default = 0)]
    pub damage : i32,
    #[var]
    #[init(default = 0)]
    pub knockback: i32,
}

#[godot_api]
impl Attack {
}
