use godot::prelude::*;
use std::ops::Deref;

struct RustFinal;

#[gdextension]
unsafe impl ExtensionLibrary for RustFinal {}

mod health_component;
mod movement_component;