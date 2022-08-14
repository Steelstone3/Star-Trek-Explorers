const SHIELD_STRENGTH: u32 = 100;
const HULL_INTEGRITY: u32 = 100;
const PHASER_MAX_DAMAGE: u32 = 5;
const PHASER_MIN_DAMAGE: u32 = 1;
const TORPEDO_MAX_DAMAGE: u32 = 10;
const TORPEDO_MIN_DAMAGE: u32 = 1;
const SHIELD_DAMAGE_MODIFIER_MAX: u32 = 5;
const SHIELD_DAMAGE_MODIFIER_MIN: u32 = 1;
const HULL_DAMAGE_MODIFIER_MAX: u32 = 5;
const HULL_DAMAGE_MODIFIER_MIN: u32 = 1;

pub struct Ship {
    pub display_symbol: char,
    pub name: String,
    pub faction: String,
    pub class: String,
    pub shield_strength: u32,
    pub hull_integrity: u32,
    pub phaser_max_damage: u32,
    pub phaser_min_damage: u32,
    pub torpedo_max_damage: u32,
    pub torpedo_min_damage: u32,
}

impl Ship {
    pub fn create_ship(display_symbol: char, name: String, faction: String, class: String) -> Ship {
        Ship {
            display_symbol,
            name,
            faction,
            class,
            shield_strength: SHIELD_STRENGTH,
            hull_integrity: HULL_INTEGRITY,
            phaser_max_damage: PHASER_MAX_DAMAGE,
            phaser_min_damage: PHASER_MIN_DAMAGE,
            torpedo_max_damage: TORPEDO_MAX_DAMAGE,
            torpedo_min_damage: TORPEDO_MIN_DAMAGE,
        }
    }

    pub fn passive_ship_scan(ship: &Ship) -> String {
        let scanned_ship = format!(
            "Scanning ship...\nShip ({}):\nName: {}\nFaction: {}\nClass: {}\n",
            ship.display_symbol, ship.name, ship.faction, ship.class
        );

        scanned_ship
    }

    pub fn aggressive_ship_scan(ship: &Ship) -> String {
        let scanned_ship = format!(
            "Scanning ship...\nShip ({}):\nName: {}\nFaction: {}\nClass: {}\nShield strength: {}\nHull integrity: {}\n",
            ship.display_symbol, ship.name, ship.faction, ship.class, ship.shield_strength, ship.hull_integrity
        );

        scanned_ship
    }

    pub fn aggressive_ships_scan(ships: &[Ship]) -> String {
        let mut scanned_ships: String = "".to_string();

        for ship in ships {
            scanned_ships += &Ship::aggressive_ship_scan(ship);
        }

        scanned_ships
    }

    pub fn fire_phasers(&self, defending_ship: &mut Ship) {
        let phaser_shield_damage = self.phaser_max_damage * SHIELD_DAMAGE_MODIFIER_MAX;
        let phaser_hull_damage = self.phaser_min_damage * HULL_DAMAGE_MODIFIER_MIN;
        let has_shield_depleted = defending_ship.shield_strength == 0;
        if phaser_shield_damage >= defending_ship.shield_strength {
            defending_ship.shield_strength = 0;
        } else if defending_ship.shield_strength > 0 {
            defending_ship.shield_strength -= phaser_shield_damage;
        }

        if has_shield_depleted && phaser_hull_damage >= defending_ship.hull_integrity {
            defending_ship.hull_integrity = 0;
        } else if has_shield_depleted && defending_ship.hull_integrity > 0 {
            defending_ship.hull_integrity -= phaser_hull_damage;
        }
    }

    pub fn fire_torpedoes(&self, defending_ship: &mut Ship) {
        let torpedo_shield_damage = self.torpedo_min_damage * SHIELD_DAMAGE_MODIFIER_MIN;
        let torpedo_hull_damage = self.torpedo_max_damage * HULL_DAMAGE_MODIFIER_MAX;
        let has_shield_depleted = defending_ship.shield_strength == 0;
        if torpedo_shield_damage >= defending_ship.shield_strength {
            defending_ship.shield_strength = 0;
        } else if defending_ship.shield_strength > 0 {
            defending_ship.shield_strength -= torpedo_shield_damage;
        }

        if has_shield_depleted && torpedo_hull_damage >= defending_ship.hull_integrity {
            defending_ship.hull_integrity = 0;
        } else if has_shield_depleted && defending_ship.hull_integrity > 0 {
            defending_ship.hull_integrity -= torpedo_hull_damage;
        }
    }
}

#[cfg(test)]
mod ship_model_should {
    use super::*;

    #[test]
    fn create_ship() {
        let starship = create_starship_fixture();

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-474661", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Defiant Class", starship.class);
        assert_eq!(SHIELD_STRENGTH, starship.shield_strength);
        assert_eq!(HULL_INTEGRITY, starship.hull_integrity);
        assert_eq!(PHASER_MAX_DAMAGE, starship.phaser_max_damage);
        assert_eq!(PHASER_MIN_DAMAGE, starship.phaser_min_damage);
        assert_eq!(TORPEDO_MAX_DAMAGE, starship.torpedo_max_damage);
        assert_eq!(TORPEDO_MIN_DAMAGE, starship.torpedo_min_damage);
    }

    #[test]
    fn be_able_to_perform_a_passive_ship_scan() {
        let starship = create_starship_fixture();

        let scanned_ship = Ship::passive_ship_scan(&starship);

        assert_eq!("Scanning ship...\nShip (S):\nName: USS Enterprise NCC-474661\nFaction: Federation\nClass: Defiant Class\n", scanned_ship);
    }

    #[test]
    fn be_able_to_perform_an_aggressive_ship_scan() {
        let starship = create_starship_fixture();

        let scanned_ship = Ship::aggressive_ship_scan(&starship);

        assert_eq!("Scanning ship...\nShip (S):\nName: USS Enterprise NCC-474661\nFaction: Federation\nClass: Defiant Class\nShield strength: 100\nHull integrity: 100\n", scanned_ship);
    }

    #[test]
    fn be_able_to_perform_an_aggressive_ships_scan() {
        let starship = vec![create_starship_fixture(), create_starship_fixture()];

        let scanned_ship = Ship::aggressive_ships_scan(&starship);

        assert_eq!("Scanning ship...\nShip (S):\nName: USS Enterprise NCC-474661\nFaction: Federation\nClass: Defiant Class\nShield strength: 100\nHull integrity: 100\nScanning ship...\nShip (S):\nName: USS Enterprise NCC-474661\nFaction: Federation\nClass: Defiant Class\nShield strength: 100\nHull integrity: 100\n", scanned_ship);
    }

    #[test]
    fn fire_phasers_at_hostile_ship() {
        let player_ship = create_starship_fixture();
        let mut hostile_ship = create_starship_fixture();

        player_ship.fire_phasers(&mut hostile_ship);

        assert_eq!(75, hostile_ship.shield_strength);
        assert_eq!(100, hostile_ship.hull_integrity);
    }

    #[test]
    fn fire_phasers_at_hostile_ship_no_shields() {
        let player_ship = create_starship_fixture();
        let mut hostile_ship = create_starship_fixture();

        player_ship.fire_phasers(&mut hostile_ship);
        player_ship.fire_phasers(&mut hostile_ship);
        player_ship.fire_phasers(&mut hostile_ship);
        player_ship.fire_phasers(&mut hostile_ship);
        player_ship.fire_phasers(&mut hostile_ship);
        player_ship.fire_phasers(&mut hostile_ship);

        assert_eq!(0, hostile_ship.shield_strength);
        assert_eq!(98, hostile_ship.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_hostile_ship() {
        let player_ship = create_starship_fixture();
        let mut hostile_ship = create_starship_fixture();

        player_ship.fire_torpedoes(&mut hostile_ship);
        player_ship.fire_torpedoes(&mut hostile_ship);

        assert_eq!(98, hostile_ship.shield_strength);
        assert_eq!(100, hostile_ship.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_hostile_ship_no_shields() {
        let player_ship = create_starship_fixture();
        let mut hostile_ship = create_starship_fixture();
        hostile_ship.shield_strength = 0;

        player_ship.fire_torpedoes(&mut hostile_ship);

        assert_eq!(0, hostile_ship.shield_strength);
        assert_eq!(50, hostile_ship.hull_integrity);
    }

    fn create_starship_fixture() -> Ship {
        Ship::create_ship(
            'S',
            "USS Enterprise NCC-474661".to_string(),
            "Federation".to_string(),
            "Defiant Class".to_string(),
        )
    }
}
