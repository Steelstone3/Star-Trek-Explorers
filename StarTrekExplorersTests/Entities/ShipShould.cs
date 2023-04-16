using StarTrekExplorers.Components.Ship.Names;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class ShipShould
    {
        private readonly IShip ship = new Ship(Faction.Federation);

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(ship.Identification);
            Assert.NotNull(ship.Systems);
        }
    }
}