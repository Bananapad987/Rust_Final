use godot::prelude::*;
use crate::shuriken::Shuriken;
use crate::utilities::deg_to_rad;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct AbilityOne{
    #[export]
    damage : i32,

    #[export]
    knockback : f32,

    #[var]
    direction : Vector2,

    #[base]
    base : Base<Node2D>,
}

#[godot_api]
impl AbilityOne {
    #[func]
    fn throw(&mut self) {
        let scene = load::<PackedScene>("res://test_scenes/shuriken.tscn");
        if let Some(marker) = self.base.get_node("ProjectileMarker".into()) {
            if let Some(node) = scene.instantiate() {
                if let Ok(mut projectile) = node.try_cast::<Shuriken>() {

                    let marker_pos: Vector2 = marker.get("global_position".into()).to();

                    projectile.bind_mut().set_damage(self.damage);
                    projectile.bind_mut().set_knockback(self.knockback);
                    projectile.bind_mut().set_direction(self.direction);

                    if let Some(mut root_node) = self.base.get_owner() {
                        let root_pos : Vector2 = root_node.get("global_position".into()).to();

                        let projectile_pos = Vector2::new(marker_pos.x - root_pos.x, marker_pos.y - root_pos.y);
                        projectile.set("global_position".into(), projectile_pos.to_variant());
                        
                        root_node.add_child(projectile.upcast());
                    }
                }
            }
        }
    }
}

#[godot_api]
impl INode2D for AbilityOne {
    fn init(base : Base<Node2D>) -> Self {
        AbilityOne {
            damage : 0,
            knockback : 0.0,
            direction : Vector2::ZERO,
            base,
        }
    }

    fn process(&mut self, _delta : f64) {
        let input = Input::singleton();
        if input.is_action_pressed("move_left".into()) {
            self.base.set("rotation".into(), deg_to_rad(180.0).to_variant());
            self.direction = Vector2::LEFT;
        }
        if input.is_action_pressed("move_right".into()) {
            self.base.set("rotation".into(), (0.0).to_variant());
            self.direction = Vector2::RIGHT;
        }
    }
}

