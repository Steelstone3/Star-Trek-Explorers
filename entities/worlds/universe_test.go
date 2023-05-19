package worlds

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/world"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructUniverse(t *testing.T) {
	// Given
	expected := constructUniverse()

	// When
	result := ConstructUniverse()

	// Then
	assert.Equal(t, expected.Stars, result.Stars)
}

func constructUniverse() Universe {
	return Universe{
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
