use godot::prelude::*;

#[derive(GodotClass)]
#[class(init)]
pub struct Attack {
    #[var]
    #[init(default = 0)]
    pub damage : i32,
    #[var]
    #[init(default = 0.0)]
    pub knockback: f32,

    #[var]
    #[init(default = Vector2::ZERO)]
    pub direction : Vector2,
}

#[godot_api]
impl Attack {
}
