use godot::prelude::*;
use godot::engine::*;
use crate::player_body;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;
use crate::utilities::deg_to_rad;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct BasicAttack{
    #[export]
    damage : i32,

    #[export]
    knockback : i32,

    #[base]
    base : Base<Area2D>,
}

#[godot_api]
impl BasicAttack {
    fn on_body_entered(&mut self, body : Gd<Node2D>) {
        if let Some(mut player_body) = body.clone().try_cast::<PlayerBody>() {
            let attack = Attack{damage : self.damage, knockback : self.knockback};
            player_body.bind_mut().damage(attack);
        }
    }
}

#[godot_api]
impl Area2DVirtual for BasicAttack {
    fn init(base : Base<Area2D>) -> Self {
        BasicAttack {
            damage : 0,
            knockback : 0,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(node) = self.base.get_node("AttackArea".into()) {
            if let Some(mut attack_area) = node.clone().try_cast::<CollisionObject2D>() {
                attack_area.set("disabled".into(), true.to_variant());
            }
        }
    }

    fn process(&mut self, delta : f64) {
        if let Some(node) = self.base.get_parent() {
            if let Some(mut player_body) = node.clone().try_cast::<PlayerBody>() {
                let curr_velocity = player_body.bind_mut().base.get_velocity();

                if curr_velocity.x < 0.0 {
                    self.base.set("rotation".into(), deg_to_rad(180.0).to_variant());
                }

                if curr_velocity.x > 0.0 {
                    self.base.set("rotation".into(), (0.0).to_variant());
                }
            }
        }
    }
}