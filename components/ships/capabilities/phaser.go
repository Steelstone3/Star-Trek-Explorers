package capabilities

type Phaser struct {
	Damage int
}

func ConstructPhaser() Phaser {
	return Phaser{
		Damage: 10,
	}
}
