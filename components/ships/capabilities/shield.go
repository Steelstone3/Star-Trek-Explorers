package capabilities

type Shield struct {
	RepairRate                 uint
	CurrentStructuralIntegrity uint
	maximumStructuralIntegrity uint
}

func ConstructShield() Shield {
	return Shield{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		maximumStructuralIntegrity: 100,
	}
}
