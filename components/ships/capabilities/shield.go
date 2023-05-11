package capabilities

type Shield struct {
	Regeneration          uint
	CurrentShieldStrength uint
	maximumShieldStrength uint
}

func ConstructShield() Shield {
	return Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		maximumShieldStrength: 100,
	}
}
