package world

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructStar(t *testing.T) {
	// Given
	expected := constructStar()

	// When
	result := ConstructStar()

	// Then
	assert.Equal(t, expected.Name, result.Name)
	assert.Equal(t, expected.Class, result.Class)
	assert.Equal(t, expected.Planets, result.Planets)
}

func constructStar() Star {
	return Star{
		Name:    "Sol",
		Class:   "S Class",
		Planets: constructPlanets(),
	}
}

func constructPlanets() []Planet {
	return []Planet{
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
		constructPlanet(),
	}
}