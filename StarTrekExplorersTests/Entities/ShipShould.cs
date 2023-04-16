using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class ShipShould
    {
        private readonly IShip ship = new Ship(1234, Faction.Federation);

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(ship.Identification);
            Assert.NotNull(ship.ShipSystems);
        }

        [Theory(Skip="reason")]
        [InlineData()]
        public void DealDamage()
        {
            // Given
        
            // When
        
            // Then
        }

        [Theory(Skip="reason")]
        [InlineData()]
        public void TakeDamage(){
            // Given

            // When

            // Then
        }
    }
}