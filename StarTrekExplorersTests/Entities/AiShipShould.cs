using Moq;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Entities.Ships;
using StarTrekExplorers.Presenters.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class AiShipShould
    {
        private readonly Mock<IPresenter> presenter = new();
        private readonly IShip ship;

        public AiShipShould()
        {
            ship = new AiShip(presenter.Object, 1234, Faction.Federation);
        }

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(ship.Identification);
            Assert.NotNull(ship.ShipSystems);
        }

        [Theory]
        [InlineData(7777, "Torpedo", 8)]
        [InlineData(4321, "Phaser", 5)]
        public void DealDamage(int seed, string weaponName, int expectedDamage)
        {
            // Given
            presenter.Setup(p => p.Print(weaponName));

            // When
            int damage = ship.DealDamage(seed);

            // Then
            Assert.Equal(expectedDamage, damage);
            presenter.VerifyAll();
        }

        [Theory]
        [InlineData(10, 90, 100)]
        [InlineData(25, 75, 100)]
        [InlineData(100, 0, 100)]
        [InlineData(110, 0, 90)]
        [InlineData(190, 0, 10)]
        [InlineData(200, 0, 0)]
        [InlineData(210, 0, 0)]
        public void TakeDamage(int damage, int remainingShield, int remainingHull)
        {
            // When
            ship.TakeDamage(damage);

            // Then
            Assert.Equal(remainingShield, ship.ShipSystems.Shield.Current);
            Assert.Equal(remainingHull, ship.ShipSystems.Hull.Current);
        }
    }
}