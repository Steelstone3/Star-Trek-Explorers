package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructPlanetName(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := PlanetName{
		PlanetName: "HAT-P-11b",
	}

	// When
	result := constructPlanetName(seed)

	// Then
	assert.Equal(t, expected, result)
}
