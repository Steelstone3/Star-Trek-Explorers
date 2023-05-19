package systems

import (
	"fmt"
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/presenters"
)

func StartCombat(game entities.Game) (entities.Game, bool) {
	// Player's Turn
	if len(game.KlingonShips) == 0 {
		// TODO move this ↓ to a presenter
		fmt.Println("Klingon's Ship/s Destroyed!")
		return game, false
	}

	var klingonShipIndex = presenters.SelectShipIndex(game.KlingonShips)
	game.KlingonShips[klingonShipIndex] = combatTurn("\n--Player Turn--", game.PlayerShip, game.KlingonShips[klingonShipIndex])
	game.KlingonShips = filterOutDestroyedShips(game.KlingonShips)

	// AI's Turn
	if len(game.KlingonShips) > 0 {
		game.PlayerShip = combatTurn("\n--AI Turn--", game.KlingonShips[0], game.PlayerShip)
	}

	if game.PlayerShip.Capabilities.Hull.CurrentStructuralIntegrity == 0 {
		// TODO move this ↓ to using a presenter
		fmt.Println("Player's Ship Destroyed!")
		return game, false
	}

	return game, true
}

func combatTurn(turn string, attackingShip ships.Ship, defendingShip ships.Ship) ships.Ship {
	// TODO move this ↓ to using the presenter
	fmt.Println(turn)
	presenters.DisplayShip(&attackingShip)
	damagedDefendingShip := attackingShip.AttackHostileShip(defendingShip)
	presenters.DisplayShip(&damagedDefendingShip)
	return damagedDefendingShip
}

func filterOutDestroyedShips(allShips []ships.Ship) []ships.Ship {
	filteredShips := make([]ships.Ship, 0)

	for _, ship := range allShips {
		if ship.Capabilities.Hull.CurrentStructuralIntegrity != 0 {
			filteredShips = append(filteredShips, ship)
		}
	}

	return filteredShips
}
