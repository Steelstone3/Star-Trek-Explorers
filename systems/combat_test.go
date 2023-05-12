package systems

// import (
// 	"github.com/stretchr/testify/assert"
// 	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
// 	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
// 	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
// 	"testing"
// )

// func TestCombatTurn(t *testing.T) {
// 	var expectedShield uint = 90
// 	var expectedHull uint = 100
// 	ship1 := constructShip()
// 	ship2 := constructShip()
// 	ship2 = CombatTurn(ship1, ship2)
	
// 	assert.Equal(t, expectedShield, ship2.Capabilities.Shield.CurrentShieldStrength)
// 	assert.Equal(t, expectedHull, ship2.Capabilities.Hull.CurrentStructuralIntegrity)
// }

// func constructShip() ships.Ship {
// 	return ships.Ship{
// 		Identification: indentifications.ShipIdentification{
// 			Name:         "Enterprise",
// 			Class:        "Galaxy",
// 			SerialNumber: "NCC-1701",
// 			Faction:      "Federation",
// 		},
// 		Capabilities: capabilities.ShipCapabilities{
// 			Shield: capabilities.Shield{
// 				Regeneration:          5,
// 				CurrentShieldStrength: 100,
// 				MaximumShieldStrength: 100,
// 			},
// 			Hull: capabilities.Hull{
// 				RepairRate:                 5,
// 				CurrentStructuralIntegrity: 100,
// 				MaximumStructuralIntegrity: 100,
// 			},
// 			Phaser: capabilities.Phaser{
// 				Damage: 10,
// 			},
// 			Torpedo: capabilities.Torpedo{
// 				Damage: 10,
// 			},
// 		},
// 	}
// }