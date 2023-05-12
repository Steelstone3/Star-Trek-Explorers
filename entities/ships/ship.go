package ships

import (
	"fmt"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
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

func (s *Ship) DisplayShip() {
	fmt.Println("")
	fmt.Print(" ", s.Identification.Name)
	fmt.Print(" ", s.Identification.Class)
	fmt.Print(" ", s.Identification.SerialNumber)
	fmt.Print(" ", s.Identification.Faction)

	fmt.Println("")
	fmt.Print("Offensive Capability || ", )
	fmt.Print("Phaser: ", s.Capabilities.Phaser.Damage)
	fmt.Print(" | Torpedo: ", s.Capabilities.Torpedo.Damage)

	fmt.Println("")
	fmt.Print("Defensive Capability || ")
	fmt.Print("Shield Strength: ", s.Capabilities.Shield.CurrentShieldStrength)
	fmt.Print(" | Hull Structrual Integrity: ", s.Capabilities.Hull.CurrentStructuralIntegrity)
	fmt.Println("")
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