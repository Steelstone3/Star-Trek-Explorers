package capabilities

import ()

type Hull struct {
	RepairRate                 uint
	CurrentStructuralIntegrity uint
	maximumStructuralIntegrity uint
}

func ConstructHull() Hull {
	return Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		maximumStructuralIntegrity: 100,
	}
}
