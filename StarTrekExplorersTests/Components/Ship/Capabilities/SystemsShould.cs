using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Capabilities
{
    public class SystemsShould
    {
        private readonly IShipSystems shipSystems = new ShipSystems();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(shipSystems.Phaser);
            Assert.NotNull(shipSystems.Torpedo);
            Assert.NotNull(shipSystems.Shield);
            Assert.NotNull(shipSystems.Hull);
        }
    }
}