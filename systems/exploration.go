package systems

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/presenters"
)

func StartExploration(game entities.Game) entities.Game {
	presenters.DisplayStars(&game.Universe.Stars)

	return game
}
