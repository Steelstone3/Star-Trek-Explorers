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

func (h *Hull) TakeHullDamage(shieldStrength uint, damage uint) Hull {
	hull := ConstructHull()

	if shieldStrength == 0 {
		if damage > h.CurrentStructuralIntegrity {
			hull.CurrentStructuralIntegrity = 0
			return hull
		}

		remainingHull := h.CurrentStructuralIntegrity - damage
		hull.CurrentStructuralIntegrity = remainingHull
		return hull
	}

	return hull
}
