using System;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorers.Systems.Interfaces;

namespace StarTrekExplorers.Systems
{
    public class Combat : ICombat
    {
        private readonly IPresenter presenter;

        public Combat(IPresenter presenter)
        {
            this.presenter = presenter;
        }

        public void Start(int seed, IShip playerShip, IShip hostileShip)
        {
            Turn(seed, playerShip, hostileShip);
            Turn(seed, hostileShip, playerShip);
        }

        private void Turn(int seed, IShip attackingShip, IShip defendingShip)
        {
            presenter.NewLine();
            presenter.ShipPresenter.PrintShipName(attackingShip);
            presenter.ShipPresenter.PrintShipOffensiveSystems(attackingShip);

            presenter.NewLine();
            int damage = attackingShip.DealDamage(seed);
            defendingShip.TakeDamage(damage);

            presenter.NewLine();
            presenter.ShipPresenter.PrintShipName(defendingShip);
            presenter.ShipPresenter.PrintShipDefensiveSystems(defendingShip);
        }
    }
}