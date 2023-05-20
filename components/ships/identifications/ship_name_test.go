package identifications

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructFederationShipName(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipName{
		Name: "Monitor",
	}

	// When
	result := constructFederationShipName(seed)

	// Then
	assert.Equal(t, expected.Name, result.Name)
}

func TestConstructKlingonShipName(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipName{
		Name: "Pagh",
	}

	// When
	result := constructKlingonShipName(seed)

	// Then
	assert.Equal(t, expected.Name, result.Name)
}
