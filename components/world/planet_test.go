package world

import(
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructPlanet(t *testing.T) {
	// Given
	expected := constructPlanet()
	
	// When
	result := ConstructPlanet()

	// Then
	assert.Equal(t, expected.Name, result.Name)
	assert.Equal(t, expected.Class, result.Class)
}

func constructPlanet() Planet {
	return Planet{
		Name:  "Earth",
		Class: "M Class",
	}
}