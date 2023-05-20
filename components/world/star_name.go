package world

import "github.com/Steelstone3/Star-Trek-Explorers/systems/generators"

type StarName struct {
	StarName string
}

func constructStarName(seed int64) StarName {
	starNames := []string{
		"Proxima Centauri (HIP 70890)",
		"TRAPPIST-1 (2MASS J23062928-0502285)",
		"Kepler-452 (HIP 104893)",
		"Kepler-186 (KOI-571)",
		"Kepler-22 (KOI-087)",
		"Gliese 581 (HD 22049)",
		"Gliese 876 (HD 217014)",
		"Gliese 667 (HD 160691)",
		"Tau Ceti (HD 10700)",
		"51 Pegasi (HD 217014)",
		"Epsilon Eridani (HD 22049)",
		"GJ 1214 (HIP 116906)",
		"GJ 436 (HIP 57087)",
		"Kepler-62 (KOI-701)",
		"Kepler-69 (KOI-172)",
		"HD 189733 (HIP 98505)",
		"HD 209458 (HIP 116748)",
		"WASP-17 (HIP 111305)",
		"WASP-12 (HIP 109378)",
		"WASP-33 (HIP 111954)",
	}

	starName := generators.GetRandomString(seed, starNames)

	return StarName{
		StarName: starName,
	}
}
