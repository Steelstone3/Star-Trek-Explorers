package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
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