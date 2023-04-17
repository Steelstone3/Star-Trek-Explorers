using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Entities.Ships
{
    public class Ship : IShip
    {
        private readonly IPresenter presenter;

        public Ship(IPresenter presenter, int seed, Faction faction)
        {
            this.presenter = presenter;
            Identification = new Identification(seed, faction);
            ShipSystems = new ShipSystems();
        }

        public IIdentification Identification { get; protected set; }
        public IShipSystems ShipSystems { get; protected set; }

        public virtual int DealDamage(int seed)
        {
            RandomGeneration rng = new();
            string[] weaponNames = new string[] { ShipSystems.Phaser.Name, ShipSystems.Torpedo.Name };
            string weaponName = weaponNames[rng.GetRandomInRange(seed, 0, weaponNames.Length)];

            presenter.Print(weaponName);

            return weaponName == "Phaser" ? ShipSystems.Phaser.DealDamage(seed) : ShipSystems.Torpedo.DealDamage(seed);
        }

        public void TakeDamage(int damage)
        {
            int remainderOfDamage = damage - ShipSystems.Shield.Current;
            ShipSystems.Shield.TakeDamage(damage);

            if (remainderOfDamage > 0)
            {
                ShipSystems.Hull.TakeDamage(remainderOfDamage);
            }
        }
    }
}