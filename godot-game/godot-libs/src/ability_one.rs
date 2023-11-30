use godot::prelude::*;
use godot::engine::*;
use crate::player_body;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;
use crate::shuriken::Shuriken;
use crate::utilities::deg_to_rad;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct AbilityOne{
    #[export]
    damage : i32,

    #[export]
    knockback : i32,

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
        if let Some(projectile_start_pos) = self.base.get_node("ProjectileMarker".into()) {
            if let Some(mut node) = scene.instantiate() {
                node.set("transform".into(), projectile_start_pos.get("global_transform".into()));

                if let Some(mut shuriken) = node.try_cast::<Shuriken>() {
                    shuriken.bind_mut().set_damage(self.damage);
                    shuriken.bind_mut().set_knockback(self.knockback);
                    shuriken.bind_mut().set_direction(self.direction);
                    godot_print!("shuriken speed: {}", shuriken.bind().get_speed());
                    if let Some(mut root_node) = self.base.get_owner() {
                        root_node.add_child(shuriken.upcast());
                    }
                }
            }
        }
    }
}

#[godot_api]
impl Node2DVirtual for AbilityOne {
    fn init(base : Base<Node2D>) -> Self {
        AbilityOne {
            damage : 0,
            knockback : 0,
            direction : Vector2::ZERO,
            base,
        }
    }

    fn process(&mut self, delta : f64) {
        if let Some(node) = self.base.get_parent() {
            if let Some(mut player_body) = node.clone().try_cast::<PlayerBody>() {
                let curr_velocity = player_body.bind_mut().base.get_velocity();

                if curr_velocity.x < 0.0 {
                    self.base.set("rotation".into(), deg_to_rad(180.0).to_variant());
                    self.direction = Vector2::LEFT;
                }

                if curr_velocity.x > 0.0 {
                    self.base.set("rotation".into(), (0.0).to_variant());
                    self.direction = Vector2::RIGHT;
                }
            }
        }
    }
}

