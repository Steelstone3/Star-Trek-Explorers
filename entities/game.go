package entities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

type Game struct {
	PlayerShip      ships.Ship
	FederationShips []ships.Ship
	KlingonShips    []ships.Ship
}

func ConstructGame() Game {
	return Game{
		PlayerShip: ships.ConstructFederationShip(),
		FederationShips: []ships.Ship{
			ships.ConstructFederationShip(),
			ships.ConstructFederationShip(),
			ships.ConstructFederationShip(),
			ships.ConstructFederationShip(),
			ships.ConstructFederationShip(),
		},
		KlingonShips: []ships.Ship{
			ships.ConstructKlingonShip(),
			ships.ConstructKlingonShip(),
			ships.ConstructKlingonShip(),
			ships.ConstructKlingonShip(),
			ships.ConstructKlingonShip(),
		},
	}
}
