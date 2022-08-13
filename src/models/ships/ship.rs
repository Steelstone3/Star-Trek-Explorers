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

    pub fn scan_ship(ship: &Ship) -> String {
        let scanned_ship = format!(
            "Scanning ship...\n Ship ({}):\nName: {}\nFaction: {}\nClass: {}",
            ship.display_symbol, ship.name, ship.faction, ship.class
        );

        scanned_ship
    }

    #[allow(dead_code)]
    pub fn fire_phasers(&mut self, mut defending_ship: Ship) -> Ship {
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

        defending_ship
    }

    #[allow(dead_code)]
    pub fn fire_torpedoes(&mut self, mut defending_ship: Ship) -> Ship {
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

        defending_ship
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
    fn scan_ship() {
        let starship = Ship {
            display_symbol: 'K',
            name: "IKS-5176 Amak".to_string(),
            faction: "Klingon Empire".to_string(),
            class: "D7 Battle Cruiser".to_string(),
            shield_strength: SHIELD_STRENGTH,
            hull_integrity: HULL_INTEGRITY,
            phaser_max_damage: PHASER_MAX_DAMAGE,
            phaser_min_damage: PHASER_MIN_DAMAGE,
            torpedo_max_damage: TORPEDO_MAX_DAMAGE,
            torpedo_min_damage: TORPEDO_MIN_DAMAGE,
        };

        let scanned_ship = Ship::scan_ship(&starship);

        assert_eq!("Scanning ship...\n Ship (K):\nName: IKS-5176 Amak\nFaction: Klingon Empire\nClass: D7 Battle Cruiser", scanned_ship);
    }

    #[test]
    fn fire_phasers_at_hostile_ship() {
        let mut player_ship = create_starship_fixture();
        let hostile_ship = create_starship_fixture();

        let damaged_ship = player_ship.fire_phasers(hostile_ship);

        assert_eq!(75, damaged_ship.shield_strength);
        assert_eq!(100, damaged_ship.hull_integrity);
    }

    #[test]
    fn fire_phasers_at_hostile_ship_no_shields() {
        let mut player_ship = create_starship_fixture();
        let hostile_ship = create_starship_fixture();

        let damaged_ship_round_1 = player_ship.fire_phasers(hostile_ship);
        let damaged_ship_round_2 = player_ship.fire_phasers(damaged_ship_round_1);
        let damaged_ship_round_3 = player_ship.fire_phasers(damaged_ship_round_2);
        let damaged_ship_round_4 = player_ship.fire_phasers(damaged_ship_round_3);
        let damaged_ship_round_5 = player_ship.fire_phasers(damaged_ship_round_4);
        let damaged_ship_round_6 = player_ship.fire_phasers(damaged_ship_round_5);

        assert_eq!(0, damaged_ship_round_6.shield_strength);
        assert_eq!(98, damaged_ship_round_6.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_hostile_ship() {
        let mut player_ship = create_starship_fixture();
        let hostile_ship = create_starship_fixture();

        let damaged_ship_round_1 = player_ship.fire_torpedoes(hostile_ship);
        let damaged_ship_round_2 = player_ship.fire_torpedoes(damaged_ship_round_1);

        assert_eq!(98, damaged_ship_round_2.shield_strength);
        assert_eq!(100, damaged_ship_round_2.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_hostile_ship_no_shields() {
        let mut player_ship = create_starship_fixture();
        let mut hostile_ship = create_starship_fixture();
        hostile_ship.shield_strength = 0;

        let damaged_ship = player_ship.fire_torpedoes(hostile_ship);

        assert_eq!(0, damaged_ship.shield_strength);
        assert_eq!(50, damaged_ship.hull_integrity);
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
