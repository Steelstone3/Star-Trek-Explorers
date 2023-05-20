package world

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructStarClass(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := StarClass{
		StarClass: "A-Type",
	}

	// When
	result := constructStarClass(seed)

	// Then
	assert.Equal(t, expected, result)
}
