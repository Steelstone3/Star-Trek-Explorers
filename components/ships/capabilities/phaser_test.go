package capabilities

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
)

func TestPhaserConstruction(t *testing.T) {
	result := ConstructPhaser()
	expected := Phaser{
		Damage: 10,
	}

	asserters.AssertEqual(expected.Damage, result.Damage)
}
