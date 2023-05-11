package capabilities

type Phaser struct {
	Damage uint
}

func ConstructPhaser() Phaser {
	return Phaser{
		Damage: 10,
	}
}
