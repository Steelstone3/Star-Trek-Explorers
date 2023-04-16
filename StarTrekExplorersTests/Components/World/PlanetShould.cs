using StarTrekExplorers.Systems;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.World
{
    public class PlanetShould
    {
        private readonly IPlanet planet = new Planet();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(planet.Name);
            Assert.NotNull(planet.Class);
        }
    }
}