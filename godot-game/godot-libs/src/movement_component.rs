use godot::prelude::*;
use godot::engine::{CharacterBody2D, CharacterBody2DVirtual};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PlayerMovementComponent {
    #[export]
    base_movespeed : f32,
    curr_movespeed : f32,

    #[export]
    base_jumpspeed : f32,
    curr_jumpspeed : f32,

    allow_move : bool,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerMovementComponent {
}

#[godot_api]
impl CharacterBody2DVirtual for PlayerMovementComponent {
    fn init(base: Base<CharacterBody2D>) -> Self {
        PlayerMovementComponent {
            base_movespeed : 0.0,
            curr_movespeed : 0.0,
            base_jumpspeed : 0.0,
            curr_jumpspeed : 0.0,
            allow_move : false,
            base,
        }
    }

    fn ready(&mut self) {
        self.curr_movespeed = self.base_movespeed;
        self.curr_jumpspeed = self.base_jumpspeed;
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction_vector = Vector2::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            direction_vector += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            direction_vector += Vector2::LEFT;
        }

        if input.is_action_pressed("move_up".into()) {
            direction_vector -= Vector2::UP;
        }
        if input.is_action_pressed("move_down".into()) {
            direction_vector -= Vector2::DOWN;
        }

        let velocity = self.base.get_velocity();
        let new_velocity = Vector2::new(direction_vector.x * self.curr_movespeed, velocity.y + 9.0 - (direction_vector.y * self.curr_jumpspeed));

        self.base.set_velocity(new_velocity);
        self.base.move_and_slide();
    }
}