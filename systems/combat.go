package systems

import(
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func CombatTurn(attackingShip ships.Ship, defendingShip ships.Ship) ships.Ship {
	attackingShip.DisplayShip()
	damagedDefendingShip := attackingShip.AttackHostileShip(defendingShip)
	damagedDefendingShip.DisplayShip()
	return damagedDefendingShip
}