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

func (s *Ship) AttackHostileShip(defendingShip Ship) Ship {
	return defendingShip
}

func (s Ship) TakeDamageToShip(damage uint) Ship {
	var remainingDamage uint = 0

	if damage > s.Capabilities.Shield.CurrentShieldStrength{
		remainingDamage = damage - s.Capabilities.Shield.CurrentShieldStrength;
	}
	
	s.Capabilities.Shield = s.Capabilities.Shield.TakeShieldDamage(damage);
	s.Capabilities.Hull = s.Capabilities.Hull.TakeHullDamage(s.Capabilities.Shield.CurrentShieldStrength, remainingDamage);

	return s
}