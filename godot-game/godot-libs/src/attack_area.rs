use godot::prelude::*;
use godot::engine::*;

#[derive(GodotClass)]
#[class(base=CollisionShape2D)]
pub struct AttackArea {
    #[base]
    base : Base<CollisionShape2D>,
}

#[godot_api]
impl AttackArea {}

#[godot_api]
impl ICollisionShape2D for AttackArea {
    fn init(base : Base<CollisionShape2D>) -> Self {
        AttackArea {
            base
        }
    }

    fn ready(&mut self) {
        self.base.set("disabled".into(), true.to_variant());
    }
}