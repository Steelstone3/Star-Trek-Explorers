using StarTrek.Contracts.World.Builders;
using StarTrek.Controllers;
using StarTrek.Controllers.World.Builders;
using Xunit;

namespace StarTrekTests.Features
{
    public class StarSystemBuilderShould
    {
        private IStarSystemBuilder _starSystemBuilder = new StarSystemBuilder();

        [Theory]
        [InlineData(0, "Earth")]
        [InlineData(2, "Venus")]
        [InlineData(50, "Earth")]
        public void GenerateAName(int id, string expectedName)
        {
            var name = _starSystemBuilder.GetName(id);

            Assert.Equal(expectedName, name);
        }

        [Theory]
        [InlineData(0, "Yellow")]
        [InlineData(2, "Red Giant")]
        [InlineData(50, "Yellow")]
        public void GenerateAnStarType(int id, string expectedStarType)
        {
            var atmosphere = _starSystemBuilder.GetType(id);

            Assert.Equal(expectedStarType, atmosphere);
        }

        [Theory]
        [InlineData(0, 26545.23)]
        [InlineData(2, 32768.89)]
        [InlineData(50, 26545.23)]
        public void GenerateAMass(int id, double expectedMass)
        {
            var mass = _starSystemBuilder.GetMass(id);

            Assert.Equal(expectedMass, mass);
        }

        [Theory]
        [InlineData(0, 56782.23)]
        [InlineData(2, 5667.89)]
        [InlineData(50, 56782.23)]
        public void GenerateADiameter(int id, double expectedDiameter)
        {
            var diameter = _starSystemBuilder.GetDiameter(id);

            Assert.Equal(expectedDiameter, diameter);
        }
    }
}