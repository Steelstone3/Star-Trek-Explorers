package entities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/identifications"
	"github.com/Steelstone3/Star-Trek-Explorers/components/world"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/worlds"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructGame(t *testing.T) {
	// Given
	expected := constructGame()

	// When
	result := ConstructGame()

	// Then
	assert.Equal(t, expected.PlayerShip, result.PlayerShip)
	assert.Equal(t, expected.FederationShips, result.FederationShips)
	assert.Equal(t, expected.KlingonShips, result.KlingonShips)
	assert.Equal(t, expected.Universe, result.Universe)
}

func constructGame() Game {
	return Game{
		PlayerShip:      constructFederationShip(),
		FederationShips: []ships.Ship{constructFederationShip(), constructFederationShip(), constructFederationShip(), constructFederationShip(), constructFederationShip()},
		KlingonShips:    []ships.Ship{constructAiKlingonShip(), constructAiKlingonShip(), constructAiKlingonShip(), constructAiKlingonShip(), constructAiKlingonShip()},
		Universe:        constructUniverse(),
	}
}

func constructFederationShip() ships.Ship {
	return ships.Ship{
		Identification: identifications.ShipIdentification{
			Name:         "Enterprise",
			Class:        "Galaxy",
			SerialNumber: "NCC-1701",
			Faction:      "Federation",
		},
		Capabilities: capabilities.ShipCapabilities{
			Shield: capabilities.Shield{
				Regeneration:          5,
				CurrentShieldStrength: 100,
				MaximumShieldStrength: 100,
			},
			Hull: capabilities.Hull{
				RepairRate:                 5,
				CurrentStructuralIntegrity: 100,
				MaximumStructuralIntegrity: 100,
			},
			Phaser: capabilities.Phaser{
				Damage: 10,
			},
			Torpedo: capabilities.Torpedo{
				Damage: 10,
			},
		},
	}
}

func constructAiKlingonShip() ships.Ship {
	return ships.Ship{
		Identification: identifications.ShipIdentification{
			Name:         "Pagh",
			Class:        "Sompek",
			SerialNumber: "IKS-2359",
			Faction:      "Klingon Empire",
		},
		Capabilities: capabilities.ShipCapabilities{
			Shield: capabilities.Shield{
				Regeneration:          5,
				CurrentShieldStrength: 100,
				MaximumShieldStrength: 100,
			},
			Hull: capabilities.Hull{
				RepairRate:                 5,
				CurrentStructuralIntegrity: 100,
				MaximumStructuralIntegrity: 100,
			},
			Phaser: capabilities.Phaser{
				Damage: 2,
			},
			Torpedo: capabilities.Torpedo{
				Damage: 2,
			},
		},
	}
}

func constructUniverse() worlds.Universe {
	return worlds.Universe{
		Stars: constructStars(),
	}
}

func constructStars() []world.Star {
	return []world.Star{
		constructStar(),
		constructStar(),
		constructStar(),
		constructStar(),
		constructStar(),
	}
}

func constructStar() world.Star {
	return world.Star{
		Name:    "Sol",
		Class:   "S Class",
		Planets: constructPlanets(),
	}
}

func constructPlanets() []world.Planet {
	return []world.Planet{
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
	}
}

func constructPlanet() world.Planet {
	return world.Planet{
		Name:  "Earth",
		Class: "M Class",
	}
}
