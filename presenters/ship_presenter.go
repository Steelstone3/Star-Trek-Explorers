package presenters

import (
	"fmt"
	"math"

	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func SelectShipIndex(ships []ships.Ship) uint {
	fmt.Println("")

	for _, ship := range ships {
		ship.DisplayShipSummary()
	}

	index := uint(math.MaxUint64)

	for index >= uint(len(ships)) {
		index = GetUintFromConsole("Select ship:")
	}

	return index
}
