package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructStar(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := Star{
		StarName:  StarName{"Kepler-452 (HIP 104893)"},
		StarClass: StarClass{"A-Type"},
	}

	// When
	result := ConstructStar(seed)

	// Then
	assert.Equal(t, expected.StarName, result.StarName)
	assert.Equal(t, expected.StarClass, result.StarClass)
	assert.NotEmpty(t, result.Planets)
}

func TestConstructRandomStars(t *testing.T) {
	// When
	stars := ConstructRandomStars()

	// Then
	assert.NotEmpty(t, stars)
}
