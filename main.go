package main

import (
	"fmt"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
)

func main() {
	result := ships.Add(2, 3)
	fmt.Println("Result:", result)
}
