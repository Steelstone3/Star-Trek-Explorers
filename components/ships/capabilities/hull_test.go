package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructHull(t *testing.T) {
	result := ConstructHull()
	expected := Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		MaximumStructuralIntegrity: 100,
	}

	assert.Equal(t, expected.RepairRate, result.RepairRate)
	assert.Equal(t, expected.CurrentStructuralIntegrity, result.CurrentStructuralIntegrity)
	assert.Equal(t, expected.MaximumStructuralIntegrity, result.MaximumStructuralIntegrity)
}
