using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.World.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.World.Names
{
    public class PlanetClassesShould
    {
        private readonly IPlanetClasses planetClasses = new PlanetClasses();

        [Theory]
        [InlineData(1234, "Class A")]
        [InlineData(3333, "Class B")]
        public void GetPlanetClass(int seed, string expectedPlanetClass)
        {
            // When
            string planetClass = planetClasses.GetPlanetClass(seed);

            // Then
            Assert.Equal(expectedPlanetClass, planetClass);
        }
    }
}