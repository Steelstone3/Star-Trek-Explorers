package capabilities

type Torpedo struct {
	Damage uint
}

func ConstructTorpedo() Torpedo {
	return Torpedo{
		Damage: 10,
	}
}
