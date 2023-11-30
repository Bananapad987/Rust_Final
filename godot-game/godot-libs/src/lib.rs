use godot::prelude::*;

struct RustFinal;

#[gdextension]
unsafe impl ExtensionLibrary for RustFinal {}

mod health_component;
mod player_body;
mod attack_struct;
mod basic_attack;
mod player_animation;
mod ability_one;
mod shuriken;