package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructPlanet(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := Planet{
		PlanetName:  PlanetName{"HAT-P-11b"},
		PlanetClass: PlanetClass{"J Class"},
	}

	// When
	result := constructPlanet(seed)

	// Then
	assert.Equal(t, expected, result)
}

func TestConstructRandomPlanets(t *testing.T) {
	// When
	planets := constructRandomPlanets()

	// Then
	assert.NotEmpty(t, planets)
}
