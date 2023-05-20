package entities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/worlds"
)

type Game struct {
	PlayerShip      ships.Ship
	FederationShips []ships.Ship
	KlingonShips    []ships.Ship
	Universe        worlds.Galaxy
}

func ConstructGame() Game {
	return Game{
		PlayerShip:      ships.ConstructFederationShip(1),
		FederationShips: ships.ConstructRandomFederationShips(),
		KlingonShips:    ships.ConstructRandomAiKlingonShips(),
		Universe:        worlds.ConstructGalaxy(),
	}
}
