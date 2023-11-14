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

                        let x_direction: i32 = (curr_velocity.x / curr_velocity.x.abs()) as i32;
                        self.base.set("parameters/Idle&Move/blend_position".into(), x_direction.to_variant());

                        if x_direction < 0 {
                            player_sprite.set_flip_h(true);
                        }
                        if x_direction > 0{
                            player_sprite.set_flip_h(false);
                        }

                        let input = Input::singleton();
                        if input.is_action_just_pressed("basic_attack".into()) {
                            self.base.set("parameters/conditions/not_attacking".into(), false.to_variant());
                            self.base.set("parameters/conditions/is_attacking".into(), true.to_variant());
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

    fn process(&mut self, delta : f64) {
        self.update_animation();
    }
}