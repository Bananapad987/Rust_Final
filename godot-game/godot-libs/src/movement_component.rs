use godot::prelude::*;
use godot::engine::{CharacterBody2D, CharacterBody2DVirtual};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PlayerMovementComponent {
    base_movespeed : f32,
    curr_movespeed : f32,
    base_jumpspeed : f32,
    curr_jumpspeed : f32,
    allow_move : bool,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerMovementComponent {
    #[func]
    fn update_position() {
        
    }
}

#[godot_api]
impl CharacterBody2DVirtual for PlayerMovementComponent {
    fn init(base: Base<CharacterBody2D>) -> Self {
        PlayerMovementComponent {
            base_movespeed : 100.0,
            curr_movespeed : 100.0,
            base_jumpspeed : 100.0,
            curr_jumpspeed : 100.0,
            allow_move : false,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut movement_vector = Vector2::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            movement_vector += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            movement_vector += Vector2::LEFT;
        }

        self.base.set_velocity(movement_vector * self.curr_movespeed);
        godot_print!("{}", (self.base.get_velocity().x).to_string());
        self.base.move_and_slide();
    }
}