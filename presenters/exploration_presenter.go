package presenters

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/world"
)

func DisplayStars(stars *[]world.Star) {
	for _, star := range *stars {
		star.DisplayStar()
	}
}
