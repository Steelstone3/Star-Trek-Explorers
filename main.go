package main

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/systems"
)

func main() {
	game := entities.ConstructGame()
	game.StartGame()

	systems.StartCombat(game)
}
