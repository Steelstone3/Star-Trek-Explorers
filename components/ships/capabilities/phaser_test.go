package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstructPhaser(t *testing.T) {
	result := ConstructPhaser()
	expected := Phaser{
		Damage: 10,
	}

	asserters.AssertEqual(t, expected.Damage, result.Damage)
}
