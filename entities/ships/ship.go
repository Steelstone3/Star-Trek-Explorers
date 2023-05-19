package ships

import (
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

func ConstructAiKlingonShip() Ship {
	return Ship{
		Identification: indentifications.ConstructKlingonShipIdentification(),
		Capabilities:   capabilities.ConstructAiShipCapabilities(),
	}
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