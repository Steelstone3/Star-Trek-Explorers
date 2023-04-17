using System.Collections.Generic;
using Spectre.Console;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;

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
    }
}