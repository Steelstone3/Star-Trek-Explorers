package capabilities

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructTorpedo(t *testing.T) {
	// Given
	expected := Torpedo{
		Damage: 10,
	}

	// When
	result := ConstructTorpedo()

	// Then
	assert.Equal(t, expected.Damage, result.Damage)
}

func TestConstructAiTorpedo(t *testing.T) {
	// Given
	expected := Torpedo{
		Damage: 2,
	}

	// When
	result := ConstructAiTorpedo()

	// Then
	assert.Equal(t, expected.Damage, result.Damage)
}
