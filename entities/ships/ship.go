package ships

import (
	"fmt"
	"strings"

	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/identifications"
	"github.com/Steelstone3/Star-Trek-Explorers/systems/generators"
)

type Ship struct {
	Identification identifications.ShipIdentification
	Capabilities   capabilities.ShipCapabilities
}

func (ship *Ship) DisplayShipSummary() {
	shipDisplayString := "{Name} {Class} {SerialNumber} {Faction}"
	shipDisplayString = strings.Replace(shipDisplayString, "{Name}", ship.Identification.ShipName.Name, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Class}", ship.Identification.ShipClass.Class, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{SerialNumber}", ship.Identification.SerialNumber.SerialNumber, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Faction}", ship.Identification.Faction.Name, 1)

	fmt.Println(shipDisplayString)
}

func (ship *Ship) DisplayShip() {
	shipDisplayString := "\n{Name} {Class} {SerialNumber} {Faction}\nOffensive Capability || Phaser: {Phaser} | Torpedo: {Torpedo}\nDefensive Capability || Shield Strength: {ShieldStrength} | Hull Structural Integrity: {HullStructuralIntegrity}"

	shipDisplayString = strings.Replace(shipDisplayString, "{Name}", ship.Identification.ShipName.Name, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Class}", ship.Identification.ShipClass.Class, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{SerialNumber}", ship.Identification.SerialNumber.SerialNumber, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Faction}", ship.Identification.Faction.Name, 1)

	shipDisplayString = strings.Replace(shipDisplayString, "{Phaser}", fmt.Sprintf("%d", ship.Capabilities.Phaser.Damage), 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Torpedo}", fmt.Sprintf("%d", ship.Capabilities.Torpedo.Damage), 1)

	shipDisplayString = strings.Replace(shipDisplayString, "{ShieldStrength}", fmt.Sprintf("%d", ship.Capabilities.Shield.CurrentShieldStrength), 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{HullStructuralIntegrity}", fmt.Sprintf("%d", ship.Capabilities.Hull.CurrentStructuralIntegrity), 1)

	fmt.Println(shipDisplayString)
}

func ConstructFederationShip(seed int64) Ship {
	return Ship{
		Identification: identifications.ConstructFederationShipIdentification(seed),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}

func ConstructKlingonShip(seed int64) Ship {
	return Ship{
		Identification: identifications.ConstructKlingonShipIdentification(seed),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}

func ConstructAiKlingonShip(seed int64) Ship {
	return Ship{
		Identification: identifications.ConstructKlingonShipIdentification(seed),
		Capabilities:   capabilities.ConstructAiShipCapabilities(),
	}
}

func ConstructRandomFederationShips() []Ship {
	federationShips := []Ship{}

	numberOfShips := generators.GenerateRandomInRange(generators.GenerateSeed(), 1, 10)

	for i := 0; i < int(numberOfShips); i++ {
		federationShips = append(federationShips, ConstructFederationShip(generators.GenerateSeed()))
	}

	return federationShips
}

func ConstructRandomAiKlingonShips() []Ship {
	aiKlingonShips := []Ship{}

	numberOfShips := generators.GenerateRandomInRange(generators.GenerateSeed(), 1, 10)

	for i := 0; i < int(numberOfShips); i++ {
		aiKlingonShips = append(aiKlingonShips, ConstructAiKlingonShip(generators.GenerateSeed()))
	}

	return aiKlingonShips
}

func (attackingShip Ship) AttackHostileShip(defendingShip Ship) Ship {
	phaserDamage := attackingShip.Capabilities.Phaser.Damage
	maximumDamage := attackingShip.Capabilities.Phaser.Damage + attackingShip.Capabilities.Torpedo.Damage
	var remainingDamage uint

	if phaserDamage > defendingShip.Capabilities.Shield.CurrentShieldStrength {
		remainingDamage = maximumDamage - defendingShip.Capabilities.Shield.CurrentShieldStrength
	}

	defendingShip.Capabilities.Shield = defendingShip.Capabilities.Shield.TakeShieldDamage(phaserDamage)
	defendingShip.Capabilities.Hull = defendingShip.Capabilities.Hull.TakeHullDamage(defendingShip.Capabilities.Shield.CurrentShieldStrength, remainingDamage)

	return defendingShip
}

func (ship Ship) IsDestroyed() bool {
	return ship.Capabilities.Hull.CurrentStructuralIntegrity == 0 && ship.Capabilities.Shield.CurrentShieldStrength == 0
}
