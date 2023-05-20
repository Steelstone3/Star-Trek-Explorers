package systems

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/presenters"
)

func StartCombat(game entities.Game) (entities.Game, bool) {
	// Player's Turn
	if !isShipFleetDestroyed(game.KlingonShips) {
		klingonShipIndex := presenters.SelectShipIndex(game.KlingonShips)
		game.KlingonShips[klingonShipIndex] = combatTurn(game.PlayerShip, game.KlingonShips[klingonShipIndex])
		game.KlingonShips = filterOutDestroyedShips(game.KlingonShips)
	}

	if isShipFleetDestroyed(game.KlingonShips) {
		presenters.PlayerWinsCombat()
		return game, false
	}

	// AI's Turn
	if !isShipFleetDestroyed(game.KlingonShips) {
		game.PlayerShip = combatTurn(game.KlingonShips[0], game.PlayerShip)
	}

	if game.PlayerShip.IsDestroyed() {
		presenters.AiWinsCombat()
		return game, false
	}

	return game, true
}

func combatTurn(attackingShip ships.Ship, defendingShip ships.Ship) ships.Ship {
	presenters.Turn()
	attackingShip.DisplayShip()
	damagedDefendingShip := attackingShip.AttackHostileShip(defendingShip)
	damagedDefendingShip.DisplayShip()
	return damagedDefendingShip
}

func filterOutDestroyedShips(allShips []ships.Ship) []ships.Ship {
	filteredShips := make([]ships.Ship, 0)

	for _, ship := range allShips {
		if !ship.IsDestroyed() {
			filteredShips = append(filteredShips, ship)
		}
	}

	return filteredShips
}

func isShipFleetDestroyed(fleet []ships.Ship) bool {
	return !(len(fleet) > 0)
}
