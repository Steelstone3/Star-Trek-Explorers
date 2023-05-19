package presenters

import (
	"fmt"
	"math"
	"strings"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func SelectShipIndex(ships []ships.Ship) uint {
	for _, ship := range ships {
		DisplayShipSummary(&ship)
	}

	var index uint = uint(math.MaxUint64)

	for index >= uint(len(ships)) {
		index = GetUintFromConsole("Select ship:")
	}

	return index
}

func DisplayShipSummary(ship *ships.Ship) {
	shipDisplayString := "{Name} {Class} {SerialNumber} {Faction}"
	shipDisplayString = strings.Replace(shipDisplayString, "{Name}", ship.Identification.Name, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Class}", ship.Identification.Class, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{SerialNumber}", ship.Identification.SerialNumber, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Faction}", ship.Identification.Faction, 1)

	fmt.Println(shipDisplayString)
}

func DisplayShip(ship *ships.Ship) {
	shipDisplayString := "\n{Name} {Class} {SerialNumber} {Faction}\nOffensive Capability || Phaser: {Phaser} | Torpedo: {Torpedo}\nDefensive Capability || Shield Strength: {ShieldStrength} | Hull Structural Integrity: {HullStructuralIntegrity}"

	shipDisplayString = strings.Replace(shipDisplayString, "{Name}", ship.Identification.Name, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Class}", ship.Identification.Class, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{SerialNumber}", ship.Identification.SerialNumber, 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Faction}", ship.Identification.Faction, 1)

	shipDisplayString = strings.Replace(shipDisplayString, "{Phaser}", fmt.Sprintf("%d", ship.Capabilities.Phaser.Damage), 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{Torpedo}", fmt.Sprintf("%d", ship.Capabilities.Torpedo.Damage), 1)

	shipDisplayString = strings.Replace(shipDisplayString, "{ShieldStrength}", fmt.Sprintf("%d", ship.Capabilities.Shield.CurrentShieldStrength), 1)
	shipDisplayString = strings.Replace(shipDisplayString, "{HullStructuralIntegrity}", fmt.Sprintf("%d", ship.Capabilities.Hull.CurrentStructuralIntegrity), 1)
	
	fmt.Println(shipDisplayString)
}