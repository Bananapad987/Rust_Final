use godot::prelude::*;
use godot::engine::*;
use crate::health_component::HealthComponent;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct RockMonster2Body {
    #[export]
    base_movespeed : f32,
    curr_movespeed : f32,

    #[export]
    base_jumpspeed : f32,
    curr_jumpspeed : f32,

    #[export]
    base_fallspeed : f32,

    #[export]
    max_fallspeed : f32,

    #[var]
    moving : bool,

    #[base]
    pub base: Base<CharacterBody2D>,
}

#[godot_api]
impl RockMonster2Body {
    #[func]
    pub fn damage(&mut self, attack : Gd<Attack>) {
        let curr_pos = self.base.get_position();
        let new_pos = curr_pos + Vector2::new(attack.bind().knockback * attack.bind().direction.x, attack.bind().knockback * attack.bind().direction.y);
        self.base.set_position(new_pos);

        if let Some(node) = self.base.get_node("HealthComponent".into()) {
            if let Ok(mut health_component) = node.try_cast::<HealthComponent>() {
                health_component.bind_mut().take_damage(attack.bind().get_damage());
            }
        }
    }

    #[func]
    pub fn change_move(&mut self) {
        self.moving = !self.moving;
    }
}

#[godot_api]
impl ICharacterBody2D for RockMonster2Body {
    fn init(base: Base<CharacterBody2D>) -> Self {
        RockMonster2Body {
            base_movespeed : 0.0,
            curr_movespeed : 0.0,
            base_jumpspeed : 0.0,
            curr_jumpspeed : 0.0,
            base_fallspeed : 0.0,
            max_fallspeed : 0.0,
            moving : false,
            base,
        }
    }

    fn ready(&mut self) {
        self.curr_movespeed = self.base_movespeed;
        self.curr_jumpspeed = self.base_jumpspeed;
    }

    fn physics_process(&mut self, _delta: f64) {
        let curr_velocity = self.base.get_velocity();
        let mut new_velocity = curr_velocity;

        if !self.base.is_on_floor() {
            new_velocity += Vector2::new(0.0, self.base_fallspeed);
        } else {
            new_velocity = Vector2::new(new_velocity.x, self.base_fallspeed);
        }
        if self.moving {
            if let Some(node) = self.base.get_node("Sprite2D".into()) {
                if let Ok(mut sprite) = node.try_cast::<Sprite2D>() {
                    let flip_h : bool = sprite.get("flip_h".into()).to();
                    
                    if flip_h {
                        new_velocity = Vector2::new(self.curr_movespeed, new_velocity.y);
                    } else {
                        new_velocity = Vector2::new(-self.curr_movespeed, new_velocity.y);
                    }
                }
            }
        } else {
            new_velocity = Vector2::new(0.0, new_velocity.y);
        }

        self.base.set_velocity(new_velocity);
        self.base.move_and_slide();
    }

}