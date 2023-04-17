using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Entities.Ships
{
    public class AiShip : Ship
    {
        private readonly IPresenter presenter;

        public AiShip(IPresenter presenter, int seed, Faction faction) : base(presenter, seed, faction)
        {
            this.presenter = presenter;
            Identification = new Identification(seed, faction);
            ShipSystems = new ShipSystems();
        }

        public override int DealDamage(int seed)
        {
            RandomGeneration rng = new();
            string[] weaponNames = new string[] { ShipSystems.Phaser.Name, ShipSystems.Torpedo.Name };
            string weaponName = weaponNames[rng.GetRandomInRange(seed, 0, weaponNames.Length)];

            presenter.Print(weaponName);

            return weaponName == "Phaser" ? ShipSystems.Phaser.DealDamage(seed) : ShipSystems.Torpedo.DealDamage(seed);
        }
    }
}