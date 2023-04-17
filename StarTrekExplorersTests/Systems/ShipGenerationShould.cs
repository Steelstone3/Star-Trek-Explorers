using System.Collections.Generic;
using System.Linq;
using Moq;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
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
            // Given
            Mock<IPresenter> presenter = new();

            // When
            IEnumerable<IShip> ships = shipGeneration.GenerateShips(presenter.Object, faction);

            // Then
            Assert.NotEmpty(ships);
            Assert.InRange(ships.Count(), 1, 10);
        }
    }
}