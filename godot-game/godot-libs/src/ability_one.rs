use godot::prelude::*;
use godot::engine::*;
use crate::player_body;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct AbilityOne{
    #[export]
    damage : i32,

    #[export]
    knockback : i32,

    #[base]
    base : Base<Area2D>,
}

#[godot_api]
impl AbilityOne {
    fn on_body_entered(&mut self, body : Gd<Node2D>) {
        if let Some(mut player_body) = body.clone().try_cast::<PlayerBody>() {
            let attack = Attack{damage : self.damage, knockback : self.knockback};
            player_body.bind_mut().damage(attack);
        }
    }
}

#[godot_api]
impl Area2DVirtual for AbilityOne {
    fn init(base : Base<Area2D>) -> Self {
        AbilityOne {
            damage : 0,
            knockback : 0,
            base,
        }
    }
}

