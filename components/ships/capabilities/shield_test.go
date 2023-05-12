package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstructShield(t *testing.T) {
	result := ConstructShield()
	expected := Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		MaximumShieldStrength: 100,
	}

	asserters.AssertEqual(t, expected.Regeneration, result.Regeneration)
	asserters.AssertEqual(t, expected.CurrentShieldStrength, result.CurrentShieldStrength)
	asserters.AssertEqual(t, expected.MaximumShieldStrength, result.MaximumShieldStrength)
}
