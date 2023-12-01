use godot::prelude::*;
use godot::engine::*;
use crate::player_body::PlayerBody;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct RockMonsterProjectile{
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
impl RockMonsterProjectile {
    #[func]
    fn on_body_entered(&mut self, body: Gd<PhysicsBody2D>) {
        godot_print!("HIT");
        if let Ok(mut player_body) = body.clone().try_cast::<PlayerBody>() {
            let curr_pos = self.base.get_position();
            let body_pos = body.get_position();
            let attack = Attack{damage : self.damage, knockback : self.knockback, direction : (curr_pos - body_pos).normalized()};
            player_body.bind_mut().damage(Gd::from_object(attack));
            self.base.queue_free();
        }
    }
}

#[godot_api]
impl IArea2D for RockMonsterProjectile {
    fn init(base : Base<Area2D>) -> Self {
        RockMonsterProjectile {
            speed : 0.0,
            direction: Vector2::ZERO,
            damage : 0,
            knockback : 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        let on_body_entered = self.base.callable("on_body_entered");
        self.base.connect("body_entered".into(), on_body_entered);

        if let Some(node) = self.base.get_node("ProjectileTimer".into()) {
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