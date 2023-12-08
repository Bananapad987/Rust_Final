use godot::prelude::*;
use godot::engine::*;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct RockMonster2Attack{
    #[export]
    damage : i32,

    #[export]
    knockback : f32,

    #[base]
    pub base : Base<Area2D>,
}

#[godot_api]
impl RockMonster2Attack {
    #[func]
    fn on_body_entered(&mut self, body: Gd<PhysicsBody2D>) {
        if let Ok(mut player_body) = body.clone().try_cast::<PlayerBody>() {
            let curr_pos = self.base.get_position();
            let body_pos = body.get_position();
            godot_print!("attack_pos: {}", curr_pos);
            godot_print!("body_pos: {}", body_pos);
            let attack = Attack{damage : self.damage, knockback : self.knockback, direction : (curr_pos - body_pos).normalized()};
            player_body.bind_mut().damage(Gd::from_object(attack));
        }
    }
}

#[godot_api]
impl IArea2D for RockMonster2Attack {
    fn init(base : Base<Area2D>) -> Self {
        RockMonster2Attack {
            damage : 0,
            knockback : 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        let on_body_entered = self.base.callable("on_body_entered");
        self.base.connect("body_entered".into(), on_body_entered);
    }
}