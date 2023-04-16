using System;
using StarTrekExplorers.Components.Ship.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public class ShipNamesShould
    {
        private readonly IShipNames shipNames = new ShipNames();

        [Theory]
        [InlineData(4321, "Centaur")]
        [InlineData(1234, "Excelsio")]
        [InlineData(1551, "Akira")]
        public void GetFederationShipName(int seed, string expectedShipName)
        {
            // When
            string shipName = shipNames.GetShipName(seed, Faction.Federation);

            // Then
            Assert.Equal(expectedShipName, shipName);
        }

        // [Theory]
        // [InlineData(4321, "Centaur")]
        // [InlineData(1234, "Excelsio")]
        // [InlineData(1551, "Akira")]
        // public void GetKlingonShipName(int seed, string expectedShipName)
        // {
        //     // When
        //     string shipName = shipNames.GetShipName(seed, Faction.Federation);

        //     // Then
        //     Assert.Equal(expectedShipName, shipName);
        // }
    }
}