use godot::prelude::*;
use godot::engine::*;
use std::vec::Vec;

#[derive(GodotClass)]
#[class(base = AnimationTree)]
struct RockMonsterAnimation {
    #[base]
    base : Base<AnimationTree>,
}

#[godot_api]
impl RockMonsterAnimation {
    #[func]
    fn attack_true(&mut self) {
        self.base.set("parameters/conditions/attack".into(), true.to_variant());
    }
}

#[godot_api]
impl IAnimationTree for RockMonsterAnimation {
    fn init(base : Base<AnimationTree>) -> Self {
        RockMonsterAnimation {
            base,
        }
    }

    fn ready(&mut self) {
        self.base.set("active".into(), true.to_variant());
        if let Some(node) = self.base.get_node("ShootTimer".into()) {
            if let Ok(mut timer) = node.try_cast::<Timer>() {
                timer.connect("timeout".into(), self.base.callable("attack_true"));
            }
        }
    }
}