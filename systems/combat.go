package systems

import (
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func StartCombat(game entities.Game) {
	//TODO If Alive Klingons && Alive player then
	//TODO Player select alive klingon vessel (UI console library)
	game.KlingonShips[0] = combatTurn(game.PlayerShip, game.KlingonShips[0])
	//TODO If Alive Klingons && Alive player then
	//TODO AI Select alive klingon vessel to attack player (random index in range from seed)
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
