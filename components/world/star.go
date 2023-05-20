package world

import (
	"fmt"
	"strings"

	"github.com/Steelstone3/Star-Trek-Explorers/systems/generators"
)

type Star struct {
	StarName  StarName
	StarClass StarClass
	Planets   []Planet
}

func (star *Star) DisplayStar() {
	starDisplayString := "\nStar: {Name} {Class}"

	starDisplayString = strings.Replace(starDisplayString, "{Name}", star.StarName.StarName, 1)
	starDisplayString = strings.Replace(starDisplayString, "{Class}", star.StarClass.StarClass, 1)

	fmt.Println(starDisplayString)

	for _, planet := range star.Planets {
		planet.DisplayPlanet()
	}
}

func ConstructStar(seed int64) Star {
	return Star{
		StarName:  constructStarName(seed),
		StarClass: constructStarClass(seed),
		Planets:   constructRandomPlanets(),
	}
}

func ConstructRandomStars() []Star {
	stars := []Star{}

	numberOfStars := generators.GenerateRandomInRange(generators.GenerateSeed(), 100, 1000)

	for i := 0; i < int(numberOfStars); i++ {
		stars = append(stars, ConstructStar(generators.GenerateSeed()))
	}

	return stars
}
