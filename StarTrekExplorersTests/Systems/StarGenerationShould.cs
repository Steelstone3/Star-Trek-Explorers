using System.Collections.Generic;
using System.Linq;
using StarTrekExplorers.Systems;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class StarGenerationShould
    {
        private readonly IStarGeneration starGeneration = new StarGeneration();

        [Fact]
        public void GenerateStars()
        {
            // When
            IEnumerable<IStar> stars = starGeneration.GenerateStars();

            // Then
            Assert.NotEmpty(stars);
            Assert.InRange(stars.Count(), 100, 500);
        }
    }
}