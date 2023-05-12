package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestShieldConstruction(t *testing.T) {
	result := ConstructShield()
	expected := Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		maximumShieldStrength: 100,
	}

	asserters.AssertEqual(t, expected.Regeneration, result.Regeneration)
	asserters.AssertEqual(t, expected.CurrentShieldStrength, result.CurrentShieldStrength)
	asserters.AssertEqual(t, expected.maximumShieldStrength, result.maximumShieldStrength)
}
