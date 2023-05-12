package main

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/systems"
)

func main() {
	federation_ship := ships.ConstructFederationShip()
	klingon_ship := ships.ConstructKlingonShip()

	klingon_ship = systems.CombatTurn(federation_ship, klingon_ship)
	federation_ship = systems.CombatTurn(klingon_ship, federation_ship)
}
