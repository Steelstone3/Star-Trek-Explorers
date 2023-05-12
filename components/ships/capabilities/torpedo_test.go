package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstructTorpedo(t *testing.T) {
	result := ConstructTorpedo()
	expected := Torpedo{
		Damage: 10,
	}

	asserters.AssertEqual(t, expected.Damage, result.Damage)
}
