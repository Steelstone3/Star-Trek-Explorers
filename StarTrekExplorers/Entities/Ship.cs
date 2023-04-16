using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;

namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        private readonly IPresenter presenter;

        public Ship(IPresenter presenter, int seed, Faction faction)
        {
            this.presenter = presenter;
            Identification = new Identification(seed, faction);
        }

        public IIdentification Identification { get; }
        public IShipSystems ShipSystems { get; } = new ShipSystems();

        public int DealDamage(int seed)
        {
            string weaponName = presenter.ShipPresenter.SelectWeapon(ShipSystems);
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