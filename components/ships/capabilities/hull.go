package capabilities

import ()

type Hull struct {
	RepairRate                 int
	CurrentStructuralIntegrity int
	maximumStructuralIntegrity int
}

func ConstructHull() Hull {
	return Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		maximumStructuralIntegrity: 100,
	}
}
