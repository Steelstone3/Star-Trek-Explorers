using System.Collections.Generic;
using System.Linq;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Systems;
using StarTrekExplorers.Systems.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class ShipGenerationShould
    {
        private readonly IShipGeneration shipGeneration = new ShipGeneration();

        [Theory]
        [InlineData(Faction.Federation)]
        [InlineData(Faction.KlingonEmpire)]
        public void GenerateShips(Faction faction)
        {
            // When
            IEnumerable<IShip> ships = shipGeneration.GenerateShips(faction);

            // Then
            Assert.NotEmpty(ships);
            Assert.InRange(ships.Count(), 1, 10);
        }
    }
}