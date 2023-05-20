package identifications

import "github.com/Steelstone3/Star-Trek-Explorers/systems/generators"

type ShipName struct {
	Name string
}

func constructFederationShipName(seed int64) ShipName {
	federationShipNames := []string{
		"Akira",
		"Archon",
		"Atlantis",
		"Avenger",
		"Aventine",
		"Bozeman",
		"Centaur",
		"Challenger",
		"Chekov",
		"Chimera",
		"Columbia",
		"Cortez",
		"Curry",
		"Dauntless",
		"Defiant",
		"Discovery",
		"Endeavour",
		"Enterprise",
		"Excalibur",
		"Excelsio",
		"Excelsior",
		"Farragut",
		"Galaxy",
		"Grissom",
		"Hood",
		"Horizon",
		"Intrepid",
		"Lakota",
		"Lexington",
		"Majestic",
		"Melbourne",
		"Merrimack",
		"Monitor",
		"Nebula",
		"Newton",
		"Nova",
		"Odyssey",
		"Pasteur",
		"Potemkin",
		"Prometheus",
		"Proxima",
		"Relativity",
		"Saratoga",
		"Stargazer",
		"Sutherland",
		"Thunderchild",
		"Titan",
		"Valiant",
		"Voyager",
		"Yorktown",
	}

	shipName := generators.GetRandomString(seed, federationShipNames)

	return ShipName{shipName}
}

func constructKlingonShipName(seed int64) ShipName {
	klingonShipNames := []string{
		"Amar",
		"B'Moth",
		"B'rel",
		"Buruk",
		"Ch'Tang",
		"Groth",
		"Heghta",
		"K'Vada",
		"K'Vort",
		"K't'inga",
		"Ki'tang",
		"Koraga",
		"M'Char",
		"NeghVar",
		"Ning'tao",
		"Orantho",
		"Pagh",
		"Rotarran",
		"Slivin",
		"Somraw",
		"T'Acog",
		"Tagana",
		"Toh'Kaht",
		"Voodieh",
		"Vor'cha",
		"Vorn",
	}

	shipName := generators.GetRandomString(seed, klingonShipNames)

	return ShipName{shipName}
}
