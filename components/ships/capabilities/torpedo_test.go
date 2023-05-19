package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructTorpedo(t *testing.T) {
	result := ConstructTorpedo()
	expected := Torpedo{
		Damage: 10,
	}

	assert.Equal(t, expected.Damage, result.Damage)
}

func TestConstructAiTorpedo(t *testing.T) {
	result := ConstructAiTorpedo()
	expected := Torpedo{
		Damage: 2,
	}

	assert.Equal(t, expected.Damage, result.Damage)
}