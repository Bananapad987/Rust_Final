use godot::prelude::*;
use godot::engine::*;
use crate::health_component::HealthComponent;
use crate::attack_struct::Attack;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerBody {
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

    #[base]
    pub base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerBody {
    #[func]
    pub fn damage(&mut self, attack : Gd<Attack>) {
        let curr_pos = self.base.get_position();
        let new_pos = curr_pos + Vector2::new(attack.bind().knockback * attack.bind().direction.x, attack.bind().knockback * (-attack.bind().direction.y - 0.3));
        self.base.set_position(new_pos);

        if let Some(node) = self.base.get_node("HealthComponent".into()) {
            if let Ok(mut health_component) = node.try_cast::<HealthComponent>() {
                health_component.bind_mut().take_damage(attack.bind().get_damage());
            }
        }
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerBody {
    fn init(base: Base<CharacterBody2D>) -> Self {
        PlayerBody {
            base_movespeed : 0.0,
            curr_movespeed : 0.0,
            base_jumpspeed : 0.0,
            curr_jumpspeed : 0.0,
            base_fallspeed : 0.0,
            max_fallspeed : 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.curr_movespeed = self.base_movespeed;
        self.curr_jumpspeed = self.base_jumpspeed;
        self.base.set_velocity(Vector2::ZERO);
    }

    fn physics_process(&mut self, delta: f64) {
        let curr_velocity = self.base.get_velocity();
        let mut new_velocity = curr_velocity;
        let curr_pos = self.base.get_position();
        let mut new_pos = curr_pos;

        if !self.base.is_on_floor() {
            new_velocity += Vector2::new(0.0, self.base_fallspeed);
        } else {
            new_velocity = Vector2::new(new_velocity.x, self.base_fallspeed);
        }

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            new_pos += Vector2::new(self.curr_movespeed * delta as f32, 0.0);
        } else if input.is_action_pressed("move_left".into()) {
            new_pos -= Vector2::new(self.curr_movespeed * delta as f32, 0.0);
        } 

        if input.is_action_pressed("move_up".into()) && self.base.is_on_floor() {
            new_velocity -= Vector2::new(0.0, self.curr_jumpspeed);
        }
        if input.is_action_pressed("move_down".into()) && new_velocity.y < self.max_fallspeed {
            new_velocity += Vector2::new(0.0, self.curr_jumpspeed);
        }

        self.base.set_velocity(new_velocity);
        self.base.set_position(new_pos);
        self.base.move_and_slide();
    }
}