package world

import "github.com/Steelstone3/Star-Trek-Explorers/systems/generators"

type StarClass struct {
	StarClass string
}

func constructStarClass(seed int64) StarClass {
	starClasses := []string{
		"O-Type",
		"B-Type",
		"A-Type",
		"F-Type",
		"G-Type",
		"K-Type",
		"M-Type",
	}

	starClass := generators.GetRandomString(seed, starClasses)

	return StarClass{
		StarClass: starClass,
	}
}
