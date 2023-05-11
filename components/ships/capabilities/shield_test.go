package capabilities

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
)

func TestShieldConstruction(t *testing.T) {
	result := ConstructShield()
	expected := Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		maximumShieldStrength: 100,
	}

	asserters.AssertEqual(expected.Regeneration, result.Regeneration)
	asserters.AssertEqual(expected.CurrentShieldStrength, result.CurrentShieldStrength)
	asserters.AssertEqual(expected.maximumShieldStrength, result.maximumShieldStrength)
}
