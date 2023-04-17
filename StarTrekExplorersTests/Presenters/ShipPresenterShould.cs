using System.Collections.Generic;
using Moq;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Presenters
{
    public class ShipPresenterShould
    {
        private const string identificationMessage = "| Ship: USS-1234 Enterprise Galaxy |";
        private readonly Mock<IShip> ship = new();
        private readonly Mock<IIdentification> identification = new();
        private readonly Mock<IPresenter> presenter = new();
        private readonly IShipPresenter shipPresenter;

        public ShipPresenterShould()
        {
            identification.Setup(i => i.Name).Returns("Enterprise");
            identification.Setup(i => i.ShipClass).Returns("Galaxy");
            identification.Setup(i => i.SerialNumber).Returns("USS-1234");
            ship.Setup(s => s.Identification).Returns(identification.Object);
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

        [Fact]
        public void PrintShipName()
        {
            // Given
            presenter.Setup(p => p.Print(identificationMessage));

            // When
            shipPresenter.PrintShipName(ship.Object);

            // Then
            presenter.VerifyAll();
        }

        [Fact]
        public void PrintShipNames()
        {
            // Given
            List<IShip> ships = new() { ship.Object, ship.Object };
            presenter.Setup(p => p.Print(identificationMessage));

            // When
            shipPresenter.PrintShipNames(ships);

            // Then
            presenter.Verify(p => p.Print(identificationMessage), Times.Exactly(2));
        }

        [Fact(Skip = "reason")]
        public void PrintShipOffensiveSystems()
        {
            // Given

            // When

            // Then
        }

        [Fact(Skip = "reason")]
        public void PrintShipDefensiveSystems()
        {
            // Given

            // When

            // Then
        }
    }
}