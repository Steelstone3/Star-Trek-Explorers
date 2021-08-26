pub struct StarShip {
    prefix: String,
    name: String,
    warp_system: WarpSystem,
    impulse_system: ImpulseSystem,
    deflector_system: DeflectorSystem,
    weapon_system: WeaponsSystem,
    system_power: SystemPower,
}

struct WarpSystem {
    name: String,
    class: i32,
    max_warp_factor: i32,
    max_warp_power: i32,
}

struct ImpulseSystem {
    name: String,
    class: i32,
    max_impulse: i32,
    max_impulse_power: i32,
}

struct DeflectorSystem {
    name: String,
    class: i32,
    max_deflector_power: i32,
}

struct WeaponsSystem {
    phaser: Phaser,
    torpedo: Torpedo,
}

struct SystemPower {
    max_total_power: i32,
    max_weapons_power: i32,
    max_shields_power: i32,
    max_engines_power: i32,
    max_systems_power: i32,
}

struct Phaser {
    phaser_name: String,
    phaser_class: i32,
    number_of_phaser: i32,
    max_phaser_firing_controller: i32,
    max_phaser_damage_shields: i32,
    min_phaser_damage_shields: i32,
    max_phaser_damage_hull: i32,
    min_phaser_damage_hull: i32,
}

struct Torpedo {
    torpedo_name: String,
    torpedo_class: i32,
    number_of_torpedoes: i32,
    number_of_torpedo_launchers: i32,
    max_torpedo_firing_controller: i32,
    max_torpedo_damage_shields: i32,
    min_torpedo_damage_shields: i32,
    max_torpedo_damage_hull: i32,
    min_torpedo_damage_hull: i32,
}

pub fn create(starship_name: String) -> StarShip {
    StarShip {
        prefix: "USS ".to_string(),
        name: starship_name,
        warp_system: setup_warp_system(),
        impulse_system: setup_impulse_system(),
        deflector_system: setup_deflector_system(),
        weapon_system: setup_weapon_system(),
        system_power: setup_system_power(),
    }
}

fn setup_warp_system() -> WarpSystem {
    WarpSystem {
        name: "Starfleet's standard issue warp drive.".to_string(),
        class: 5,
        max_warp_factor: 8,
        max_warp_power: 20,
    }
}

fn setup_impulse_system() -> ImpulseSystem {
    ImpulseSystem {
        name: "Starfleet's standard issue impulse engine.".to_string(),
        class: 5,
        max_impulse: 120,
        max_impulse_power: 40,
    }
}

fn setup_deflector_system() -> DeflectorSystem {
    DeflectorSystem {
        name: "Starfleet's standard issue deflector dish".to_string(),
        class: 5,
        max_deflector_power: 15,
    }
}

fn setup_system_power() -> SystemPower {
    SystemPower {
        max_total_power: 100,
        max_weapons_power: 50,
        max_shields_power: 75,
        max_engines_power: 50,
        max_systems_power: 25,
    }
}

fn setup_weapon_system() -> WeaponsSystem{
    WeaponsSystem{
        phaser: setup_phaser(),
        torpedo: setup_torpedo(),
    }
}

fn setup_phaser() -> Phaser {
    Phaser {
        phaser_name: "Starfleet's standard issue phaser array".to_string(),
        phaser_class: 5,
        number_of_phaser: 10,
        max_phaser_firing_controller: 4,
        max_phaser_damage_shields: 200,
        min_phaser_damage_shields: 125,
        max_phaser_damage_hull: 50,
        min_phaser_damage_hull: 10,
    }
}

fn setup_torpedo() -> Torpedo {
    Torpedo {
        torpedo_name: "Starfleet's standard issue torpedo launcher".to_string(),
        torpedo_class: 5,
        number_of_torpedoes: 40,
        number_of_torpedo_launchers: 2,
        max_torpedo_firing_controller: 1,
        max_torpedo_damage_shields: 50,
        min_torpedo_damage_shields: 10,
        max_torpedo_damage_hull: 200,
        min_torpedo_damage_hull: 125,
    }
}

pub fn details(starship: StarShip) -> String {
    return [starship.prefix.as_str(), starship.name.as_str()].join(" ");
}

#[cfg(test)]
mod model_starship_should {
    use super::*;

    #[test]
    fn generate_a_setup_warp_system() {
        let warp_system = setup_warp_system();
        assert_eq!(warp_system.name, "Starfleet's standard issue warp drive.");
        assert_eq!(warp_system.class, 5);
        assert_eq!(warp_system.max_warp_factor, 8);
        assert_eq!(warp_system.max_warp_power, 20);
    }

    #[test]
    fn generate_a_setup_impulse_system() {
        let impulse_system = setup_impulse_system();
        assert_eq!(impulse_system.name, "Starfleet's standard issue impulse engine.");
        assert_eq!(impulse_system.class, 5);
        assert_eq!(impulse_system.max_impulse, 120);
        assert_eq!(impulse_system.max_impulse_power, 40);
    }

    #[test]
    fn generate_a_setup_deflector_system() {
        let deflector_system = setup_deflector_system();
        assert_eq!(deflector_system.name, "Starfleet's standard issue deflector dish");
        assert_eq!(deflector_system.class, 5);
        assert_eq!(deflector_system.max_deflector_power, 15);
    }

    #[test]
    fn generate_a_setup_weapon_system() {
        let weapon_system = setup_weapon_system();
        assert_eq!(weapon_system.phaser.max_phaser_damage_hull, 50);
        assert_eq!(weapon_system.phaser.max_phaser_damage_shields,200);
        assert_eq!(weapon_system.phaser.max_phaser_firing_controller,4);
        assert_eq!(weapon_system.phaser.min_phaser_damage_hull,10);
        assert_eq!(weapon_system.phaser.min_phaser_damage_shields,125);
        assert_eq!(weapon_system.phaser.number_of_phaser,10);
        assert_eq!(weapon_system.phaser.phaser_class,5);
        assert_eq!(weapon_system.phaser.phaser_name,"Starfleet's standard issue phaser array");

        assert_eq!(weapon_system.torpedo.max_torpedo_damage_hull,200);
        assert_eq!(weapon_system.torpedo.max_torpedo_damage_shields,50);
        assert_eq!(weapon_system.torpedo.max_torpedo_firing_controller,1);
        assert_eq!(weapon_system.torpedo.min_torpedo_damage_hull,125);
        assert_eq!(weapon_system.torpedo.min_torpedo_damage_shields,10);
        assert_eq!(weapon_system.torpedo.number_of_torpedo_launchers,2);
        assert_eq!(weapon_system.torpedo.number_of_torpedoes,40);
        assert_eq!(weapon_system.torpedo.torpedo_class,5);
        assert_eq!(weapon_system.torpedo.torpedo_name,"Starfleet's standard issue torpedo launcher");
    }

    #[test]
    fn generate_a_setup_system_power() {
        let system_power = setup_system_power();
        assert_eq!(system_power.max_total_power, 100);
        assert_eq!(system_power.max_weapons_power, 50);
        assert_eq!(system_power.max_shields_power, 75);
        assert_eq!(system_power.max_engines_power, 50);
        assert_eq!(system_power.max_systems_power, 25);
    }

    #[test]
    fn create_a_starship() {
        let warp_system = setup_warp_system();
        let impulse_system = setup_impulse_system();
        let deflector_system = setup_deflector_system();
        let weapon_system = setup_weapon_system();
        let system_power = setup_system_power();

        let starship = create("Enterprise".to_string());

        assert_eq!(starship.prefix, "USS ");
        assert_eq!(starship.name, "Enterprise");

        assert_eq!(starship.warp_system.name, warp_system.name);
        assert_eq!(starship.warp_system.class, warp_system.class);
        assert_eq!(starship.warp_system.max_warp_factor, warp_system.max_warp_factor);
        assert_eq!(starship.warp_system.max_warp_power, warp_system.max_warp_power);

        assert_eq!(starship.impulse_system.name, impulse_system.name);
        assert_eq!(starship.impulse_system.class, impulse_system.class);
        assert_eq!(starship.impulse_system.max_impulse, impulse_system.max_impulse);
        assert_eq!(starship.impulse_system.max_impulse_power, impulse_system.max_impulse_power);

        assert_eq!(starship.deflector_system.name, deflector_system.name);
        assert_eq!(starship.deflector_system.class, deflector_system.class);
        assert_eq!(starship.deflector_system.max_deflector_power, deflector_system.max_deflector_power);

        assert_eq!(starship.weapon_system.phaser.max_phaser_damage_hull, weapon_system.phaser.max_phaser_damage_hull);
        assert_eq!(starship.weapon_system.phaser.max_phaser_damage_shields, weapon_system.phaser.max_phaser_damage_shields);
        assert_eq!(starship.weapon_system.phaser.max_phaser_firing_controller, weapon_system.phaser.max_phaser_firing_controller);
        assert_eq!(starship.weapon_system.phaser.min_phaser_damage_hull, weapon_system.phaser.min_phaser_damage_hull);
        assert_eq!(starship.weapon_system.phaser.min_phaser_damage_shields, weapon_system.phaser.min_phaser_damage_shields);
        assert_eq!(starship.weapon_system.phaser.number_of_phaser, weapon_system.phaser.number_of_phaser);
        assert_eq!(starship.weapon_system.phaser.phaser_class, weapon_system.phaser.phaser_class);
        assert_eq!(starship.weapon_system.phaser.phaser_name, weapon_system.phaser.phaser_name);

        assert_eq!(starship.weapon_system.torpedo.max_torpedo_damage_hull, weapon_system.torpedo.max_torpedo_damage_hull);
        assert_eq!(starship.weapon_system.torpedo.max_torpedo_damage_shields, weapon_system.torpedo.max_torpedo_damage_shields);
        assert_eq!(starship.weapon_system.torpedo.max_torpedo_firing_controller, weapon_system.torpedo.max_torpedo_firing_controller);
        assert_eq!(starship.weapon_system.torpedo.min_torpedo_damage_hull, weapon_system.torpedo.min_torpedo_damage_hull);
        assert_eq!(starship.weapon_system.torpedo.min_torpedo_damage_shields, weapon_system.torpedo.min_torpedo_damage_shields);
        assert_eq!(starship.weapon_system.torpedo.number_of_torpedo_launchers, weapon_system.torpedo.number_of_torpedo_launchers);
        assert_eq!(starship.weapon_system.torpedo.number_of_torpedoes, weapon_system.torpedo.number_of_torpedoes);
        assert_eq!(starship.weapon_system.torpedo.torpedo_class, weapon_system.torpedo.torpedo_class);
        assert_eq!(starship.weapon_system.torpedo.torpedo_name, weapon_system.torpedo.torpedo_name);

        assert_eq!(starship.system_power.max_total_power, system_power.max_total_power);
        assert_eq!(starship.system_power.max_weapons_power, system_power.max_weapons_power);
        assert_eq!(starship.system_power.max_shields_power, system_power.max_shields_power);
        assert_eq!(starship.system_power.max_engines_power, system_power.max_engines_power);
        assert_eq!(starship.system_power.max_systems_power, system_power.max_systems_power);
    }

    #[test]
    fn provide_starship_details() {
        let enterprise = StarShip {
            prefix: "USS".to_string(),
            name: "Enterprise".to_string(),
            warp_system: setup_warp_system(),
            impulse_system: setup_impulse_system(),
            deflector_system: setup_deflector_system(),
            weapon_system: setup_weapon_system(),
            system_power: setup_system_power(),
        };

        let details = details(enterprise);

        assert_eq!(details, "USS Enterprise");
    }
}