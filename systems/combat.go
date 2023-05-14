package systems

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func StartCombat(game entities.Game) {
	game.KlingonShips[0] = combatTurn(game.PlayerShip, game.KlingonShips[0])
	game.PlayerShip = combatTurn(game.KlingonShips[0], game.PlayerShip)

	endCombat(game)
}

func combatTurn(attackingShip ships.Ship, defendingShip ships.Ship) ships.Ship {
	attackingShip.DisplayShip()
	damagedDefendingShip := attackingShip.AttackHostileShip(defendingShip)
	damagedDefendingShip.DisplayShip()
	return damagedDefendingShip
}

func endCombat(game entities.Game) {

}
