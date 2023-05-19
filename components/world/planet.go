package world

type Planet struct {
	Name string
	Class string
}

func ConstructPlanet() Planet {
	return Planet{
		Name:  "Earth",
		Class: "M Class",
	}
}
