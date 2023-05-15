package main

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/presenters"
	"github.com/Steelstone3/Star-Trek-Explorers/systems"
)

func main() {
	presenters.StartUi()
	game := entities.ConstructGame()

	

	// presenters.DisplayList()
	game.StartGame()

	systems.StartCombat(game)
}
