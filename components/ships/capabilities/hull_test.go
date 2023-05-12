package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestHullConstruction(t *testing.T) {
	result := ConstructHull()
	expected := Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		MaximumStructuralIntegrity: 100,
	}

	asserters.AssertEqual(t, expected.RepairRate, result.RepairRate)
	asserters.AssertEqual(t, expected.CurrentStructuralIntegrity, result.CurrentStructuralIntegrity)
	asserters.AssertEqual(t, expected.MaximumStructuralIntegrity, result.MaximumStructuralIntegrity)
}
