package ships

import (
	"fmt"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"strings"
)

type Ship struct {
	Identification indentifications.ShipIdentification
	Capabilities   capabilities.ShipCapabilities
}

func ConstructFederationShip() Ship {
	return Ship{
		Identification: indentifications.ConstructFederationShipIdentification(),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}

func ConstructKlingonShip() Ship {
	return Ship{
		Identification: indentifications.ConstructKlingonShipIdentification(),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}

// TODO create a table using library
func (s *Ship) DisplayShip() {
	ship := "{Name} {Class} {SerialNumber} {Faction}\nOffensive Capability || Phaser: {Phaser} | Torpedo: {Torpedo}\nDefensive Capability || Shield Strength: {ShieldStrength} | Hull Structural Integrity: {HullStructuralIntegrity}"

	ship = strings.Replace(ship, "{Name}", s.Identification.Name, 1)
	ship = strings.Replace(ship, "{Class}", s.Identification.Class, 1)
	ship = strings.Replace(ship, "{SerialNumber}", s.Identification.SerialNumber, 1)
	ship = strings.Replace(ship, "{Faction}", s.Identification.Faction, 1)

	ship = strings.Replace(ship, "{Phaser}", fmt.Sprintf("%d", s.Capabilities.Phaser.Damage), 1)
	ship = strings.Replace(ship, "{Torpedo}", fmt.Sprintf("%d", s.Capabilities.Torpedo.Damage), 1)

	ship = strings.Replace(ship, "{ShieldStrength}", fmt.Sprintf("%d", s.Capabilities.Shield.CurrentShieldStrength), 1)
	ship = strings.Replace(ship, "{HullStructuralIntegrity}", fmt.Sprintf("%d", s.Capabilities.Hull.CurrentStructuralIntegrity), 1)
	
	fmt.Println(ship)
}

func (attackingShip Ship) AttackHostileShip(defendingShip Ship) Ship {
	var phaserDamage = attackingShip.Capabilities.Phaser.Damage
	var maximumDamage = attackingShip.Capabilities.Phaser.Damage + attackingShip.Capabilities.Torpedo.Damage
	var remainingDamage uint = 0

	if phaserDamage > defendingShip.Capabilities.Shield.CurrentShieldStrength {
		remainingDamage = maximumDamage - defendingShip.Capabilities.Shield.CurrentShieldStrength
	}

	defendingShip.Capabilities.Shield = defendingShip.Capabilities.Shield.TakeShieldDamage(phaserDamage)
	defendingShip.Capabilities.Hull = defendingShip.Capabilities.Hull.TakeHullDamage(defendingShip.Capabilities.Shield.CurrentShieldStrength, remainingDamage)

	return defendingShip
}
