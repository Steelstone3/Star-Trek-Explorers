package entities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/worlds"
)

type Game struct {
	PlayerShip      ships.Ship
	FederationShips []ships.Ship
	KlingonShips    []ships.Ship
	Universe worlds.Universe
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
			ships.ConstructAiKlingonShip(),
			ships.ConstructAiKlingonShip(),
			ships.ConstructAiKlingonShip(),
			ships.ConstructAiKlingonShip(),
			ships.ConstructAiKlingonShip(),
		},
		Universe: worlds.ConstructUniverse(),
	}
}