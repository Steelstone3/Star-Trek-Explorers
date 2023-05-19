package main

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/systems"
)

func main() {
	game := entities.ConstructGame()
	var isInCombat bool = true

	for isInCombat {
		game = systems.StartExploration(game)
		game, isInCombat = systems.StartCombat(game)
	}
}
