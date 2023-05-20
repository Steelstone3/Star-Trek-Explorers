package world

import "github.com/Steelstone3/Star-Trek-Explorers/systems/generators"

type PlanetClass struct {
	PlanetClass string
}

func constructPlanetClass(seed int64) PlanetClass {
	planetClasses := []string{
		"D Class",
		"H Class",
		"J Class",
		"K Class",
		"L Class",
		"M Class",
		"N Class",
		"R Class",
		"T Class",
		"Y Class",
	}

	planetClass := generators.GetRandomString(seed, planetClasses)

	return PlanetClass{
		PlanetClass: planetClass,
	}
}
