using System.Collections.Generic;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;

namespace StarTrekExplorers.Presenters
{
    public class ShipPresenter : IShipPresenter
    {
        private readonly IPresenter presenter;

        public ShipPresenter(IPresenter presenter)
        {
            this.presenter = presenter;
        }

        public string SelectWeapon(IShipSystems shipSystems)
        {
            const string message = "Select Weapon:";
            List<string> options = new() { shipSystems.Phaser.Name, shipSystems.Torpedo.Name };

            string weaponName = presenter.SelectString(message, options);

            presenter.Print(weaponName);

            return weaponName;
        }

        public void PrintShipNames(IEnumerable<IShip> ships)
        {
            foreach (IShip ship in ships)
            {
                PrintShipName(ship);
            }
        }

        public void PrintShipName(IShip ship)
        {
            presenter.Print($"| Ship: {ship.Identification.SerialNumber} {ship.Identification.Name} {ship.Identification.ShipClass} |");
        }

    }
}