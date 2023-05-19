package capabilities

type Phaser struct {
	Damage uint
}

func ConstructPhaser() Phaser {
	return Phaser{
		Damage: 10,
	}
}

func ConstructAiPhaser() Phaser {
	return Phaser{
		Damage: 2,
	}
}