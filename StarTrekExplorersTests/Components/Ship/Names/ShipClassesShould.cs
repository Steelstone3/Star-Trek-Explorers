using StarTrekExplorers.Components.Ship.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public class ShipClassesShould
    {
        private readonly IShipClasses shipClasses = new ShipClasses();

        [Theory]
        [InlineData(4321, "Defiant")]
        [InlineData(1234, "Saber")]
        [InlineData(1551, "Galaxy")]
        public void GetFederationShipClass(int seed, string expectedShipClass)
        {
            // When
            string shipClass = shipClasses.GetShipClass(seed, Faction.Federation);

            // Then
            Assert.Equal(expectedShipClass, shipClass);
        }

        [Theory]
        [InlineData(4321, "K'mpec")]
        [InlineData(1234, "Lotl'eh")]
        [InlineData(1551, "Negh'Var")]
        public void GetKlingonShipClass(int seed, string expectedShipClass)
        {
            // When
            string shipClass = shipClasses.GetShipClass(seed, Faction.KlingonEmpire);

            // Then
            Assert.Equal(expectedShipClass, shipClass);
        }
    }
}