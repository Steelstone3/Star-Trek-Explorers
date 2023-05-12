package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructShield(t *testing.T) {
	result := ConstructShield()
	expected := Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		MaximumShieldStrength: 100,
	}

	assert.Equal(t, expected.Regeneration, result.Regeneration)
	assert.Equal(t, expected.CurrentShieldStrength, result.CurrentShieldStrength)
	assert.Equal(t, expected.MaximumShieldStrength, result.MaximumShieldStrength)
}
