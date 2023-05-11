package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstruction(t *testing.T) {
	result := Construct()
	expected := Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		maximumStructuralIntegrity: 100,
	}

	asserters.AssertEqual(expected, result)
}
