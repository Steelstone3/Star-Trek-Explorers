package worlds

import(
	"github.com/Steelstone3/Star-Trek-Explorers/components/world"
)

type Universe struct {
	Stars []world.Star
}

func ConstructUniverse() Universe{
	return Universe{
		Stars: []world.Star{
			world.ConstructStar(),
			world.ConstructStar(),
			world.ConstructStar(),
			world.ConstructStar(),
			world.ConstructStar(),
		},
	}
}
