use crate::models::ship::meta_data::MetaData;
use crate::models::ship::system_power::SystemPower;
use crate::models::ship::systems::Systems;

pub struct Starship {
    pub(crate) meta_data: MetaData,
    pub(crate) systems: Systems,
    pub(crate) system_power: SystemPower,
}

impl Starship {
    pub fn create(meta_data: MetaData,
                  systems: Systems,
                  system_power: SystemPower) -> Starship {
        return Starship {
            meta_data,
            systems,
            system_power,
        };
    }
}