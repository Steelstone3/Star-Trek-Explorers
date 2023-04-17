using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.World.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.World.Names
{
    public class StarClassesShould
    {
        private readonly IStarClasses starClasses = new StarClasses();

        [Theory]
        [InlineData(1234, "Class A")]
        [InlineData(3333, "Class B")]
        public void GetPlanetClass(int seed, string expectedStarClass)
        {
            // When
            string starClass = starClasses.GetStarClass(seed);

            // Then
            Assert.Equal(expectedStarClass, starClass);
        }
    }
}