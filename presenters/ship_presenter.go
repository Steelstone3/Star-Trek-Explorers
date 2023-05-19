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

func DisplayShipSummary(s *ships.Ship) {
	ship := "{Name} {Class} {SerialNumber} {Faction}"
	ship = strings.Replace(ship, "{Name}", s.Identification.Name, 1)
	ship = strings.Replace(ship, "{Class}", s.Identification.Class, 1)
	ship = strings.Replace(ship, "{SerialNumber}", s.Identification.SerialNumber, 1)
	ship = strings.Replace(ship, "{Faction}", s.Identification.Faction, 1)

	fmt.Println(ship)
}

func DisplayShip(s *ships.Ship) {
	ship := "\n{Name} {Class} {SerialNumber} {Faction}\nOffensive Capability || Phaser: {Phaser} | Torpedo: {Torpedo}\nDefensive Capability || Shield Strength: {ShieldStrength} | Hull Structural Integrity: {HullStructuralIntegrity}"

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