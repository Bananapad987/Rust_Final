use godot::prelude::*;
use godot::engine::*;
use crate::monster_body::MonsterBody;
use crate::attack_struct::Attack;
use crate::rock_monster_2_body::RockMonster2Body;

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
    knockback : f32,

    #[base]
    pub base : Base<Area2D>,
}

#[godot_api]
impl Shuriken {
    #[func]
    fn on_body_entered(&mut self, body: Gd<PhysicsBody2D>) {
        godot_print!("HIT");
        if let Ok(mut monster_body) = body.clone().try_cast::<MonsterBody>() {
            let curr_pos = self.base.get_position();
            let body_pos = body.get_position();
            let attack = Attack{damage : self.damage, knockback : self.knockback, direction : (curr_pos - body_pos).normalized()};
            monster_body.bind_mut().damage(Gd::from_object(attack));
            self.base.queue_free();
        }
        
        if let Ok(mut monster_body) = body.clone().try_cast::<RockMonster2Body>() {
            let curr_pos = self.base.get_position();
            let body_pos = body.get_position();
            let attack = Attack{damage : self.damage, knockback : self.knockback, direction : (curr_pos - body_pos).normalized()};
            monster_body.bind_mut().damage(Gd::from_object(attack));
        }
    }
}

#[godot_api]
impl IArea2D for Shuriken {
    fn init(base : Base<Area2D>) -> Self {
        Shuriken {
            speed : 0.0,
            direction: Vector2::RIGHT,
            damage : 0,
            knockback : 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        let on_body_entered = self.base.callable("on_body_entered");
        self.base.connect("body_entered".into(), on_body_entered);

        if let Some(node) = self.base.get_node("Timer".into()) {
            if let Ok(mut timer) = node.clone().try_cast::<Timer>() {
                timer.connect("timeout".into(), self.base.callable("queue_free"));
            }
        }
    }

    fn process(&mut self, delta : f64) {
        let curr_pos = self.base.get_position();
        let new_pos = curr_pos + (self.direction * self.speed * delta as f32);
        self.base.set_position(new_pos);
    }
}