package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructStarName(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := StarName{
		StarName: "Kepler-452 (HIP 104893)",
	}

	// When
	result := constructStarName(seed)

	// Then
	assert.Equal(t, expected, result)
}
