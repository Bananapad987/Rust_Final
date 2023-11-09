use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct HealthComponent {
    #[export]
    max_health: i32,
    curr_health: i32,

    #[base]
    base: Base<Node2D>,
}
#[godot_api]
impl HealthComponent {
    #[func]
    fn damage(&mut self, damage : i32) {
        self.curr_health -= damage;
    }
}

#[godot_api]
impl Node2DVirtual for HealthComponent {
    fn init(base : Base<Node2D>) -> Self {
        HealthComponent {
            max_health : 0,
            curr_health : 0,
            base,
        }
    }

    fn ready(&mut self) {
        self.curr_health = self.max_health;
    }

    fn process(&mut self, delta : f64) {
        godot_print!("{}", self.curr_health.to_string());
        self.damage(1);
    }
}