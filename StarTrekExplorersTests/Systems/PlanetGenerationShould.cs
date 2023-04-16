using System.Collections.Generic;
using System.Linq;
using StarTrekExplorers.Systems;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class PlanetGenerationShould
    {
        private readonly IPlanetGeneration planetGeneration = new PlanetGeneration();

        [Fact]
        public void GeneratePlanet()
        {
            // When
            IEnumerable<IPlanet> planets = planetGeneration.GeneratePlanets();

            // Then
            Assert.NotEmpty(planets);
            Assert.InRange(planets.Count(), 1, 10);
        }
    }
}