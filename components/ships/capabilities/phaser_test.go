package capabilities

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructPhaser(t *testing.T) {
	// Given
	expected := Phaser{
		Damage: 10,
	}

	// When
	result := ConstructPhaser()

	// Then
	assert.Equal(t, expected.Damage, result.Damage)
}

func TestConstructAiPhaser(t *testing.T) {
	// Given
	expected := Phaser{
		Damage: 2,
	}

	// When
	result := ConstructAiPhaser()

	// Then
	assert.Equal(t, expected.Damage, result.Damage)
}
