package capabilities

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructShield(t *testing.T) {
	// Given
	expected := constructShield()

	// When
	result := ConstructShield()

	// Then
	assert.Equal(t, expected.Regeneration, result.Regeneration)
	assert.Equal(t, expected.CurrentShieldStrength, result.CurrentShieldStrength)
	assert.Equal(t, expected.MaximumShieldStrength, result.MaximumShieldStrength)
}

func TestTakeShieldDamage(t *testing.T) {
	// Given
	var damage uint = 10
	var expectedRemainingShield uint = 90
	shield := constructShield()

	// When
	shield = shield.TakeShieldDamage(damage)

	// Then
	assert.Equal(t, expectedRemainingShield, shield.CurrentShieldStrength)
}

func TestTakeCriticalShieldDamage(t *testing.T) {
	// Given
	var damage uint = 101
	var expectedRemainingShield uint = 0
	shield := constructShield()

	shield = shield.TakeShieldDamage(damage)

	assert.Equal(t, expectedRemainingShield, shield.CurrentShieldStrength)
}

func TestTakeShieldDamageAcceptance(t *testing.T) {
	// Given
	var damage uint = 10
	var expectedRemainingShield uint = 50
	shield := constructShield()

	// Then
	shield = shield.TakeShieldDamage(damage)
	shield = shield.TakeShieldDamage(damage)
	shield = shield.TakeShieldDamage(damage)
	shield = shield.TakeShieldDamage(damage)
	shield = shield.TakeShieldDamage(damage)

	assert.Equal(t, expectedRemainingShield, shield.CurrentShieldStrength)
}

func constructShield() Shield {
	return Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		MaximumShieldStrength: 100,
	}
}
