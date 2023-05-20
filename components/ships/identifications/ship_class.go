package identifications

import "github.com/Steelstone3/Star-Trek-Explorers/systems/generators"

type ShipClass struct {
	Class string
}

func constructFederationShipClass(seed int64) ShipClass {
	federationShipClasses := []string{
		"Galaxy",
		"Intrepid",
		"Defiant",
		"Sovereign",
		"Oberth",
		"Nova",
		"Saber",
		"Miranda",
		"Constellation",
		"Cheyenne",
		"Dakota",
		"Prometheus",
		"Nebula",
		"Luna",
		"Akira",
		"Excelsior",
		"Ambassador",
		"Odyssey",
	}

	shipClass := generators.GetRandomString(seed, federationShipClasses)

	return ShipClass{
		Class: shipClass,
	}
}

func constructKlingonShipClass(seed int64) ShipClass {
	klingonShipClasses := []string{
		"Negh'Var",
		"Gel'joQ",
		"B'rel",
		"Felg'ra",
		"K'mpec",
		"K'Vort",
		"Qethla",
		"Torath",
		"Vor'cha",
		"De'nat",
		"DughHegh",
		"Fel'keth",
		"Goralis",
		"Jen'thar",
		"K't'inga",
		"Lotl'eh",
		"Ngapej",
		"Pa'chag",
		"QaDlej",
		"Vodleq",
		"Tormag",
		"Ro'qul",
		"BaH'reth",
		"HajHal",
		"Kel'var",
		"Qa'cheng",
		"Sa'var",
		"To'beq",
		"Yotwl",
		"Bach'chunD",
		"DeSjoH",
		"Po'gach",
		"Sompek",
		"Tro'Qa",
		"Bla'koth",
		"DorHub",
		"Drenok",
		"Qlj'tagh",
		"Vel'tas",
		"Ver'graH",
	}

	shipClass := generators.GetRandomString(seed, klingonShipClasses)

	return ShipClass{
		Class: shipClass,
	}
}
