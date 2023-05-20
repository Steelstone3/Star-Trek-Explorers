package worlds

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructGalaxy(t *testing.T) {
	// When
	result := ConstructGalaxy()

	// Then
	assert.NotEmpty(t, result.Stars)
}
