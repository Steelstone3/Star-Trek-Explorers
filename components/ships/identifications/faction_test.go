package identifications

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructFederationFaction(t *testing.T) {
	// Given
	expected := Faction{
		Name: "Federation",
	}

	// When
	result := constructFederationFaction()

	// Then
	assert.Equal(t, expected.Name, result.Name)
}

func TestConstructKlingonFaction(t *testing.T) {
	// Given
	expected := Faction{
		Name: "Klingon Empire",
	}

	// When
	result := constructKlingonFaction()

	// Then
	assert.Equal(t, expected.Name, result.Name)
}
