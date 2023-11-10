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

    #[export]
    base_fallspeed : f32,

    #[export]
    max_fallspeed : f32,

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
            base_fallspeed : 0.0,
            max_fallspeed : 0.0,
            allow_move : false,
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
}