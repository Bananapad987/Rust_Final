use godot::prelude::*;
use godot::engine::*;
use crate::monster_body::MonsterBody;
use crate::attack_struct::Attack;
use crate::utilities::deg_to_rad;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct BasicAttack{
    #[export]
    damage : i32,

    #[export]
    knockback : f32,

    #[base]
    base : Base<Area2D>,
}

#[godot_api]
impl BasicAttack {
    #[func]
    fn on_body_entered(&mut self, body: Gd<PhysicsBody2D>) {
        if let Ok(mut monster_body) = body.clone().try_cast::<MonsterBody>() {
            let curr_pos = self.base.get_position();
            let body_pos = body.get_position();
            let attack = Attack{damage : self.damage, knockback : self.knockback, direction : (curr_pos - body_pos).normalized()};
            monster_body.bind_mut().damage(Gd::from_object(attack));
        }
    }
}

#[godot_api]
impl IArea2D for BasicAttack {
    fn init(base : Base<Area2D>) -> Self {
        BasicAttack {
            damage : 0,
            knockback : 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        let on_body_entered = self.base.callable("on_body_entered");
        self.base.connect("body_entered".into(), on_body_entered);

        if let Some(node) = self.base.get_node("AttackArea".into()) {
            if let Ok(mut attack_area) = node.clone().try_cast::<CollisionObject2D>() {
                attack_area.set("disabled".into(), true.to_variant());
            }
        }
    }

    fn process(&mut self, _delta : f64) {
        let input = Input::singleton();
        if input.is_action_pressed("move_left".into()) {
            self.base.set("rotation".into(), deg_to_rad(180.0).to_variant());
        }
        if input.is_action_pressed("move_right".into()) {
            self.base.set("rotation".into(), (0.0).to_variant());
        }
    }
}