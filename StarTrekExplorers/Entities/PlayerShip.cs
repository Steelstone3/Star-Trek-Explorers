using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Ships;
using StarTrekExplorers.Presenters.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class PlayerShip : Ship
    {
        private readonly IPresenter presenter;

        public PlayerShip(IPresenter presenter, int seed, Faction faction) : base(presenter, seed, faction)
        {
            this.presenter = presenter;
            Identification = new Identification(seed, faction);
            ShipSystems = new ShipSystems();
        }

        public override int DealDamage(int seed)
        {
            string weaponName = presenter.ShipPresenter.SelectWeapon(ShipSystems);
            return weaponName == "Phaser" ? ShipSystems.Phaser.DealDamage(seed) : ShipSystems.Torpedo.DealDamage(seed);
        }
    }
}