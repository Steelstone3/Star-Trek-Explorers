using System.Collections.Generic;
using Moq;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Presenters
{
    public class ShipPresenterShould
    {
        private readonly Mock<IPresenter> presenter = new();
        private readonly IShipPresenter shipPresenter;

        public ShipPresenterShould()
        {
            shipPresenter = new ShipPresenter(presenter.Object);
        }

        [Fact]
        public void SelectAWeapon()
        {
            // Given
            const string weaponName = "Phaser";
            const string otherWeaponName = "Torpedo";
            Mock<IShipSystems> shipSystems = new();
            shipSystems.Setup(ss => ss.Phaser.Name).Returns(weaponName);
            shipSystems.Setup(ss => ss.Torpedo.Name).Returns(otherWeaponName);
            List<string> options = new() { shipSystems.Object.Phaser.Name, shipSystems.Object.Torpedo.Name };
            presenter.Setup(p => p.SelectString("Select Weapon:", options)).Returns(weaponName);
            presenter.Setup(p => p.Print(weaponName));

            // When
            shipPresenter.SelectWeapon(shipSystems.Object);

            // Then
            presenter.VerifyAll();
        }
    }
}