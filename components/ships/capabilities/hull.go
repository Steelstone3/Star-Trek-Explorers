package capabilities

type Hull struct {
	RepairRate                 uint
	CurrentStructuralIntegrity uint
	MaximumStructuralIntegrity uint
}

func ConstructHull() Hull {
	return Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		MaximumStructuralIntegrity: 100,
	}
}
