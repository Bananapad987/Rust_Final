use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct HealthComponent {
    #[export]
    pub max_health: i32,
    pub curr_health: i32,

    #[base]
    base : Base<Node2D>
}

#[godot_api]
impl HealthComponent {
    #[func]
    pub fn take_damage(&mut self, damage : i32) {
        self.curr_health -= damage;
        godot_print!("curr_hp: {}", self.curr_health);
    }
}

#[godot_api]
impl INode2D for HealthComponent {
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

    fn process(&mut self, _delta : f64) {
        if self.curr_health <= 0 {
            if let Some(mut node) = self.base.get_owner() {
                node.queue_free();
            }
        }
    }
}