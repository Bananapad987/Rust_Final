use godot::prelude::*;
use godot::engine::*;
use crate::player_body::PlayerBody;
use std::vec::Vec;

#[derive(GodotClass)]
#[class(base = AnimationTree)]
struct PlayerAnimation {
    #[base]
    base : Base<AnimationTree>,
}

#[godot_api]
impl PlayerAnimation {
    #[func]
    fn update_animation(&mut self) {
        if let Some(node) = self.base.get_parent() {
            if let Some(mut player_body) = node.clone().try_cast::<PlayerBody>() {
                if let Some(node) = self.base.get_node("../PlayerSprite".into()) {
                    if let Some(mut player_sprite) = node.clone().try_cast::<Sprite2D>() {
                        let curr_velocity = player_body.bind_mut().base.get_velocity();

                        if curr_velocity.x < 0.0 {
                            player_sprite.set_flip_h(true);
                        }
                        if curr_velocity.x > 0.0 {
                            player_sprite.set_flip_h(false);
                        } else {
                            self.base.set("parameters/conditions/not_moving".into(), true.to_variant());
                            self.base.set("parameters/conditions/is_moving".into(), false.to_variant());
                        }

                        if curr_velocity.y < 0.0 {
                            self.base.set("parameters/conditions/not_jumping".into(), false.to_variant());
                            self.base.set("parameters/conditions/is_jumping".into(), true.to_variant());
                            self.base.set("parameters/conditions/is_falling".into(), false.to_variant());
                            self.base.set("parameters/conditions/not_falling".into(), true.to_variant());
                        } else if curr_velocity.y > 0.0 {
                            self.base.set("parameters/conditions/is_falling".into(), true.to_variant());
                            self.base.set("parameters/conditions/not_falling".into(), false.to_variant());
                            self.base.set("parameters/conditions/not_jumping".into(), true.to_variant());
                            self.base.set("parameters/conditions/is_jumping".into(), false.to_variant());
                        } else {
                            self.base.set("parameters/conditions/is_jumping".into(), false.to_variant());
                            self.base.set("parameters/conditions/not_jumping".into(), true.to_variant());
                            self.base.set("parameters/conditions/not_falling".into(), true.to_variant());
                            self.base.set("parameters/conditions/is_falling".into(), false.to_variant());
                        }

                        let input = Input::singleton();
                        if input.is_action_pressed("move_right".into()) || input.is_action_pressed("move_left".into()) {
                            self.base.set("parameters/conditions/not_moving".into(), false.to_variant());
                            self.base.set("parameters/conditions/is_moving".into(), true.to_variant());
                        }
                        if input.is_action_just_pressed("basic_attack".into()) {
                            self.base.set("parameters/conditions/is_attacking".into(), true.to_variant());
                        }

                        if input.is_action_just_pressed("ability_one".into()) {
                            self.base.set("parameters/conditions/ability_one".into(), true.to_variant());
                        }
                    }
                }
            }
        }
    }
}

#[godot_api]
impl AnimationTreeVirtual for PlayerAnimation {
    fn init(base : Base<AnimationTree>) -> Self {
        PlayerAnimation {
            base,
        }
    }

    fn ready(&mut self) {
        self.base.set("active".into(), true.to_variant());
        self.base.set("parameters/conditions/not_attacking".into(), false.to_variant());
    }

    fn process(&mut self, delta : f64) {
        self.update_animation();
    }
}