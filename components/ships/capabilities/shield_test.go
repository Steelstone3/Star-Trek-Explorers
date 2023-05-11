package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestShieldConstruction(t *testing.T) {
	result := ConstructShield()
	expected := Shield{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		maximumStructuralIntegrity: 100,
	}

	asserters.AssertEqual(expected.RepairRate, result.RepairRate)
	asserters.AssertEqual(expected.CurrentStructuralIntegrity, result.CurrentStructuralIntegrity)
	asserters.AssertEqual(expected.maximumStructuralIntegrity, result.maximumStructuralIntegrity)
}
