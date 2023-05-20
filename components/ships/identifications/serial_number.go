package identifications

import (
	"strconv"

	"github.com/Steelstone3/Star-Trek-Explorers/systems/generators"
)

type SerialNumber struct {
	SerialNumber string
}

func constructFederationSerialNumber(seed int64) SerialNumber {
	prefix := "USS-"

	number := generators.GenerateRandomInRange(seed, 1000, 99999)

	numberString := strconv.Itoa(int(number))

	return SerialNumber{
		SerialNumber: prefix + numberString,
	}
}

func constructKlingonSerialNumber(seed int64) SerialNumber {
	prefix := "IKS-"

	number := generators.GenerateRandomInRange(seed, 1000, 99999)

	numberString := strconv.Itoa(int(number))

	return SerialNumber{
		SerialNumber: prefix + numberString,
	}
}
