using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class PlayerShip : IShip
    {
        private readonly IPresenter presenter;

        public PlayerShip(IPresenter presenter, int seed, Faction faction)
        {
            this.presenter = presenter;
            Identification = new Identification(seed, faction);
            ShipSystems = new ShipSystems();
        }

        public IIdentification Identification { get; protected set; }
        public IShipSystems ShipSystems { get; protected set; }

        public virtual int DealDamage(int seed)
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