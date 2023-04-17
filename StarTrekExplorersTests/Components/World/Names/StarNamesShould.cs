using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.World.Names;
using Xunit;

namespace StarTrekExplorersTests.Components.World.Names
{
    public class StarNamesShould
    {
        private readonly IStarNames starNames = new StarNames();

        [Theory]
        [InlineData(1234, "Betelgeuse")]
        [InlineData(111, "Risa")]
        public void GetPlanetClass(int seed, string expectedStarClass)
        {
            // When
            string starClass = starNames.GetName(seed);

            // Then
            Assert.Equal(expectedStarClass, starClass);
        }
    }
}