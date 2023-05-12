package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructPhaser(t *testing.T) {
	result := ConstructPhaser()
	expected := Phaser{
		Damage: 10,
	}

	assert.Equal(t, expected.Damage, result.Damage)
}
