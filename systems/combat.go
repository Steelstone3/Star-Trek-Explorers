package systems

import(
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func CombatTurn(attackingShip ships.Ship, defendingShip ships.Ship) ships.Ship {
	defendingShip = attackingShip.AttackHostileShip(defendingShip)

	return defendingShip
}