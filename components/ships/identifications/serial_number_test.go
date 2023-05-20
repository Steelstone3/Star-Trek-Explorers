package identifications

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructFederationSerialNumber(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := SerialNumber{
		SerialNumber: "USS-37482",
	}

	// When
	result := constructFederationSerialNumber(seed)

	// Then
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}

func TestConstructKlingonSerialNumber(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := SerialNumber{
		SerialNumber: "IKS-37482",
	}

	// When
	result := constructKlingonSerialNumber(seed)

	// Then
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}
