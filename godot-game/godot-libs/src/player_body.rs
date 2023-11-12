use godot::prelude::*;
use godot::engine::*;
use crate::health_component;
use crate::health_component::HealthComponent;
use crate::attack_struct::Attack;
use std::ops::*;

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
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerBody {
    pub fn damage(&mut self, attack : Attack) {
        if let Some(node) = self.base.get_node("./HealthComponent".into()) {
            if let Some(mut health_component) = node.clone().try_cast::<HealthComponent>() {
                health_component.bind_mut().take_damage(attack.damage);
            }
        }
    }

    fn update_animation(&mut self) {
        if let Some(node) = self.base.get_node("./AnimationTree".into()) {
            if let Some(mut animation_tree) = node.clone().try_cast::<AnimationTree>() {
                if self.base.get_velocity() == Vector2::ZERO {
                    animation_tree.set("parameters/conditions/is_idle".into(), true.to_variant());
                    animation_tree.set("parameters/conditions/is_moving".into(), false.to_variant());
                } else {
                    animation_tree.set("parameters/conditions/is_idle".into(), false.to_variant());
                    animation_tree.set("parameters/conditions/is_moving".into(), true.to_variant());
                }

                let input = Input::singleton();

                if input.is_action_just_pressed("basic_attack".into()) {
                    animation_tree.set("parameters/conditions/is_attacking".into(), true.to_variant());
                    animation_tree.set("parameters/conditions/is_moving".into(), false.to_variant());
                    animation_tree.set("parameters/conditions/is_idle".into(), false.to_variant());
                } else {
                    animation_tree.set("parameters/conditions/is_attacking".into(), false.to_variant());
                }

                let curr_velocity = self.base.get_velocity();
                animation_tree.set("parameters/Idle/blend_position".into(), curr_velocity.to_variant());
                animation_tree.set("parameters/Move/blend_position".into(), curr_velocity.to_variant());
                animation_tree.set("parameters/Basic_Attack/blend_position".into(), curr_velocity.to_variant());
            }
        }
    }
}

#[godot_api]
impl CharacterBody2DVirtual for PlayerBody {
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
    }

    fn physics_process(&mut self, _delta: f64) {
        let velocity = self.base.get_velocity();
        let mut new_x_velocity = 0.0;
        let mut new_y_velocity = velocity.y;

        if !self.base.is_on_floor() {
            new_y_velocity += 20.0;
        }

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            new_x_velocity = self.curr_movespeed;
        }
        if input.is_action_pressed("move_left".into()) {
            new_x_velocity = - self.curr_movespeed;
        }

        if input.is_action_pressed("move_up".into()) && self.base.is_on_floor() {
            new_y_velocity -= self.curr_jumpspeed;
        }
        if input.is_action_pressed("move_down".into()) && new_y_velocity < self.max_fallspeed {
            new_y_velocity += self.base_fallspeed;
        }

        let new_velocity = Vector2::new(new_x_velocity, new_y_velocity);

        self.base.set_velocity(new_velocity);
        self.base.move_and_slide();
    }

    fn process(&mut self, delta : f64) {
        self.update_animation();
    }
}