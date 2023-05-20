package worlds

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/world"
)

type Galaxy struct {
	Stars []world.Star
}

func ConstructGalaxy() Galaxy {
	return Galaxy{
		Stars: world.ConstructRandomStars(),
	}
}
