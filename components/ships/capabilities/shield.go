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

func (s *Shield) TakeShieldDamage(damage uint) Shield {
	if damage > s.CurrentShieldStrength{
		shield := ConstructShield()
		shield.CurrentShieldStrength = 0
		return shield
	}

	var remainingShield = s.CurrentShieldStrength - damage;
	shield := ConstructShield()
	shield.CurrentShieldStrength = remainingShield
	return shield
}
