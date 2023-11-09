use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct HealthComponent {
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
            max_health : 100,
            curr_health : 100,
            base,
        }
    }
}