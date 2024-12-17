use bevy::prelude::*;
pub mod player;
pub mod wall;

#[derive(Component)]
pub struct Pva {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,

    pub movable: bool,
    pub size: Vec2, // size used for collisions
    pub is_on_ground: bool,
}

fn update_pva(mut query: Query<&mut Pva>, time: Res<Time>) {
    for mut pva in &mut query {
        pva.update(time.delta_secs());
    }
}

fn update_transform(mut query: Query<(&Pva, &mut Transform)>) {
    for (pva, mut transform) in &mut query {
        transform.translation = pva.position;
    }
}

impl Pva {
    fn update(&mut self, delta: f32) {
        if self.movable {
            if self.velocity.y != 0.0 {
                self.is_on_ground = false;
            }
            self.position += self.velocity * delta;
            self.velocity += self.acceleration * delta;
        }
    }

    pub fn default_gravity() -> Self {
        Self {
            acceleration: Vec3::new(0.0, -3000.0, 0.0),
            movable: true,
            ..default()
        }
    }

    pub fn collision_detection(&self, other: &Self) -> bool {
        let self_half_size = self.size / 2.0;
        let other_half_size = other.size / 2.0;
        if self.position.x - self_half_size.x < other.position.x + other_half_size.x
            && self.position.x + self_half_size.x > other.position.x - other_half_size.x
            && self.position.y - self_half_size.y < other.position.y + other_half_size.y
            && self.position.y + self_half_size.y > other.position.y - other_half_size.y
        {
            true
        } else {
            false
        }
    }
}

fn collisions(mut query: Query<&mut Pva>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([mut pva1, mut pva2]) = iter.fetch_next() {
        if (pva1.movable != pva2.movable) && pva1.collision_detection(&pva2) {
            // the "!=" is here to do xor, to not do collisions on 2 movable objects (for now)
            if pva1.movable {
                if pva1.position.y > pva2.position.y + pva2.size.y / 2.0 {
                    if pva1.velocity.y < 0.0 {
                        pva1.velocity.y = 0.0;
                        pva1.is_on_ground = true;
                    }
                    pva1.position.y = pva2.position.y + pva2.size.y / 2.0 + pva1.size.y / 2.0;
                } else if pva1.position.y < pva2.position.y - pva2.size.y / 2.0 {
                    if pva1.velocity.y > 0.0 {
                        pva1.velocity.y = 0.0;
                    }
                    pva1.position.y = pva2.position.y - pva2.size.y / 2.0 - pva1.size.y / 2.0;
                } else if pva1.position.x > pva2.position.x + pva2.size.x / 2.0 {
                    if pva1.velocity.x < 0.0 {
                        pva1.velocity.x = 0.0;
                    }
                    pva1.position.x = pva2.position.x + pva2.size.x / 2.0 + pva1.size.x / 2.0;
                } else if pva1.position.x < pva2.position.x - pva2.size.x / 2.0 {
                    if pva1.velocity.x > 0.0 {
                        pva1.velocity.x = 0.0;
                    }
                    pva1.position.x = pva2.position.x - pva2.size.x / 2.0 - pva1.size.x / 2.0;
                }
            } else {
                if pva2.position.y > pva1.position.y + pva1.size.y / 2.0 {
                    if pva2.velocity.y < 0.0 {
                        pva2.velocity.y = 0.0;
                    }
                    pva2.position.y = pva1.position.y + pva1.size.y / 2.0 + pva2.size.y / 2.0;
                } else if pva2.position.y < pva1.position.y - pva1.size.y / 2.0 {
                    if pva2.velocity.y > 0.0 {
                        pva2.velocity.y = 0.0;
                    }
                    pva2.position.y = pva1.position.y - pva1.size.y / 2.0 - pva2.size.y / 2.0;
                } else if pva2.position.x > pva1.position.x + pva1.size.x / 2.0 {
                    if pva2.velocity.x < 0.0 {
                        pva2.velocity.x = 0.0;
                    }
                    pva2.position.x = pva1.position.x + pva1.size.x / 2.0 + pva2.size.x / 2.0;
                } else if pva2.position.x < pva1.position.x - pva1.size.x / 2.0 {
                    if pva2.velocity.x > 0.0 {
                        pva2.velocity.x = 0.0;
                    }
                    pva2.position.x = pva1.position.x - pva1.size.x / 2.0 - pva2.size.x / 2.0;
                }
            }
        }
    }
}

impl Default for Pva {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,

            movable: false,
            size: Vec2::ZERO,
            is_on_ground: false,
        }
    }
}

pub struct PvaPlugin;

impl Plugin for PvaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collisions)
            .add_systems(PreUpdate, update_pva)
            .add_systems(PostUpdate, update_transform);
    }
}
