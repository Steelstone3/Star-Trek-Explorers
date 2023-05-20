package identifications

type Faction struct {
	Name string
}

func constructFederationFaction() Faction {
	return Faction{
		Name: "Federation",
	}
}

func constructKlingonFaction() Faction {
	return Faction{
		Name: "Klingon Empire",
	}
}
