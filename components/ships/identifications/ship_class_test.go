package identifications

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructFederationShipClass(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipClass{
		Class: "Defiant",
	}

	// When
	result := constructFederationShipClass(seed)

	// Then
	assert.Equal(t, expected.Class, result.Class)
}

func TestConstructKlingonShipClass(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipClass{
		Class: "B'rel",
	}

	// When
	result := constructKlingonShipClass(seed)

	// Then
	assert.Equal(t, expected.Class, result.Class)
}
