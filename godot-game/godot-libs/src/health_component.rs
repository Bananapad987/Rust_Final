use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct HealthComponent {
    #[export]
    pub max_health: i32,
    pub curr_health: i32,
}

#[godot_api]
impl HealthComponent {
    #[func]
    pub fn take_damage(&mut self, damage : i32) {
        self.curr_health -= damage;
    }
}

#[godot_api]
impl Node2DVirtual for HealthComponent {
    fn init(_base : Base<Node2D>) -> Self {
        HealthComponent {
            max_health : 0,
            curr_health : 0,
        }
    }

    fn ready(&mut self) {
        self.curr_health = self.max_health;
    }
}