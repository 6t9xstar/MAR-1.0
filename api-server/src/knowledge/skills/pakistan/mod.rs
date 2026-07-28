pub mod pakistan_law;
pub mod education;
pub mod business;
pub mod agriculture;
pub mod healthcare;
pub mod islamic;
pub mod government;
pub mod geography;
pub mod culture;

use crate::knowledge::skills::registry::SkillRegistry;

pub fn register_all(registry: &SkillRegistry) {
    registry.register(pakistan_law::skill());
    registry.register(education::skill());
    registry.register(business::skill());
    registry.register(agriculture::skill());
    registry.register(healthcare::skill());
    registry.register(islamic::skill());
    registry.register(government::skill());
    registry.register(geography::skill());
    registry.register(culture::skill());
}
