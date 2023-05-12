package capabilities

type Shield struct {
	Regeneration          uint
	CurrentShieldStrength uint
	MaximumShieldStrength uint
}

func ConstructShield() Shield {
	return Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		MaximumShieldStrength: 100,
	}
}
