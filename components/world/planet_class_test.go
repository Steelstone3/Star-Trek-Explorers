package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructPlanetClass(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := PlanetClass{
		PlanetClass: "J Class",
	}

	// When
	result := constructPlanetClass(seed)

	// Then
	assert.Equal(t, expected, result)
}
