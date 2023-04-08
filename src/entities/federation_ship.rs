use crate::{entities::ship::Ship, components::ship::name::federation_ship_name::FederationShipName};

struct FederationShip {
    name: FederationShipName,
}

impl Ship for FederationShip {
    fn display_ship_name(&self) {
        println!("{}", self.name)
    }
}

#[cfg(test)]
mod ship_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
