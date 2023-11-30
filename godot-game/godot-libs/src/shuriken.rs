use godot::prelude::*;
use godot::engine::*;
use crate::player_body;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Shuriken{
    #[export]
    speed: f32,

    #[var]
    direction: Vector2,

    #[var]
    damage : i32,

    #[var]
    knockback : i32,

    #[base]
    base : Base<Area2D>,
}

#[godot_api]
impl Shuriken {
    #[func]
    fn on_Shuriken_body_entered(&mut self, body : Gd<Node2D>) {
        if let Some(mut player_body) = body.clone().try_cast::<PlayerBody>() {
            let attack = Attack{damage : self.damage, knockback : self.knockback};
            player_body.bind_mut().damage(attack);
        }

        self.base.queue_free();
    }

    #[func]
    fn remove(&mut self) {
        self.base.queue_free();
    }
}

#[godot_api]
impl Area2DVirtual for Shuriken {
    fn init(base : Base<Area2D>) -> Self {
        Shuriken {
            speed : 0.0,
            direction: Vector2::RIGHT,
            damage : 0,
            knockback : 0,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(node) = self.base.get_node("Timer".into()) {
            if let Some(mut timer) = node.clone().try_cast::<Timer>() {
                timer.connect("timeout".into(), self.base.callable("remove"));
            }
        }
    }

    fn process(&mut self, delta : f64) {
        let curr_pos = self.base.get_position();
        let new_pos = curr_pos + (self.direction * self.speed * delta as f32);
        self.base.set_position(new_pos);
    }
}