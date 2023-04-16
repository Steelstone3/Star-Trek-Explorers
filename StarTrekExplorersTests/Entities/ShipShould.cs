using Moq;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class ShipShould
    {
        private readonly Mock<IShipPresenter> shipPresenter = new();
        private readonly Mock<IPresenter> presenter = new();
        private readonly IShip ship;

        public ShipShould()
        {
            presenter.Setup(p => p.ShipPresenter).Returns(shipPresenter.Object);
            ship = new Ship(presenter.Object, 1234, Faction.Federation);
        }

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(ship.Identification);
            Assert.NotNull(ship.ShipSystems);
        }

        [Theory]
        [InlineData(1234, "Phaser", 6)]
        [InlineData(4321, "Torpedo", 5)]
        public void DealDamage(int seed, string weaponName, int expectedDamage)
        {
            // Given
            shipPresenter.Setup(sp => sp.SelectWeapon(ship.ShipSystems)).Returns(weaponName);

            // When
            int damage = ship.DealDamage(seed);

            // Then
            Assert.Equal(expectedDamage, damage);
            shipPresenter.VerifyAll();
        }

        // [Theory(Skip = "reason")]
        // [InlineData()]
        // public void TakeDamage()
        // {
        //     // Given

        //     // When

        //     // Then
        // }
    }
}