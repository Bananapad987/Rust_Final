use godot::prelude::*;
use crate::rock_monster_projectile::RockMonsterProjectile;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct RockMonsterAttack{
    #[export]
    damage : i32,

    #[export]
    knockback : f32,

    #[var]
    direction : Vector2,

    #[base]
    base : Base<Node2D>,
}

#[godot_api]
impl RockMonsterAttack {
    #[func]
    fn shoot(&mut self) {
        let scene = load::<PackedScene>("res://test_scenes/rock_monster_projectile.tscn");
        if let Some(marker) = self.base.get_node("ProjectileMarker".into()) {
            if let Some(node) = scene.instantiate() {
                if let Ok(mut projectile) = node.try_cast::<RockMonsterProjectile>() {

                    let marker_pos: Vector2 = marker.get("global_position".into()).to();

                    projectile.bind_mut().set_damage(self.damage);
                    projectile.bind_mut().set_knockback(self.knockback);
                    projectile.bind_mut().set_direction(self.direction);

                    if let Some(mut root_node) = self.base.get_owner() {
                        let root_pos : Vector2 = root_node.get("global_position".into()).to();

                        let projectile_pos = marker_pos - root_pos;
                        projectile.set("global_position".into(), projectile_pos.to_variant());
                        
                        root_node.add_child(projectile.upcast());
                    }
                }
            }
        }
    }
}

#[godot_api]
impl INode2D for RockMonsterAttack {
    fn init(base : Base<Node2D>) -> Self {
        RockMonsterAttack {
            damage : 0,
            knockback : 0.0,
            direction : Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(projectile_marker) = self.base.get_node("ProjectileMarker".into()) {
            let marker_pos : Vector2 = projectile_marker.get("global_position".into()).to();
            let curr_pos : Vector2 = self.base.get("global_position".into()).to();
            self.direction = (marker_pos - curr_pos).normalized();
        }
    }
}

