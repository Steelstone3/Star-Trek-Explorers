package world

type Star struct {
	Name string
	Class string
	Planets []Planet
}

func ConstructStar() Star {
	return Star{
		Name:    "Sol",
		Class:   "S Class",
		Planets: []Planet{
			ConstructPlanet(),
			ConstructPlanet(),
			ConstructPlanet(),
			ConstructPlanet(),
			ConstructPlanet(),
		},
	}
}