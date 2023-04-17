using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.World.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.World.Names
{
    public class PlanetNamesShould
    {
        private readonly IPlanetNames planetNames = new PlanetNames();

        [Theory]
        [InlineData(1234, "Mars")]
        [InlineData(4321, "Earth")]
        public void GetName(int seed, string expectedPlanetName)
        {
            // When
            string planetName = planetNames.GetName(seed);

            // Then
            Assert.Equal(expectedPlanetName, planetName);
        }
    }
}