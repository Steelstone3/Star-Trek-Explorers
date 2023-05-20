package world

import (
	"fmt"
	"strings"

	"github.com/Steelstone3/Star-Trek-Explorers/systems/generators"
)

type Planet struct {
	PlanetName  PlanetName
	PlanetClass PlanetClass
}

func (planet *Planet) DisplayPlanet() {
	planetDisplayString := "Planet: {Name} {Class}"

	planetDisplayString = strings.Replace(planetDisplayString, "{Name}", planet.PlanetName.PlanetName, 1)
	planetDisplayString = strings.Replace(planetDisplayString, "{Class}", planet.PlanetClass.PlanetClass, 1)

	fmt.Println(planetDisplayString)
}

func constructPlanet(seed int64) Planet {
	return Planet{
		PlanetName:  constructPlanetName(seed),
		PlanetClass: constructPlanetClass(seed),
	}
}

func constructRandomPlanets() []Planet {
	planets := []Planet{}

	numberOfStars := generators.GenerateRandomInRange(generators.GenerateSeed(), 1, 10)

	for i := 0; i < int(numberOfStars); i++ {
		planets = append(planets, constructPlanet(generators.GenerateSeed()))
	}

	return planets
}
