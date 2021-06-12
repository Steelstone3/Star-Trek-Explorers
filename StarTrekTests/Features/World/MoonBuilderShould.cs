using StarTrek.Controllers.World.Builders;
using Xunit;

namespace StarTrekTests.Features
{
    public class MoonBuilderShould
    {
        private MoonBuilder _moonBuilder = new MoonBuilder();

        [Theory]
        [InlineData(0, "Luna")]
        [InlineData(1, "Titan")]
        [InlineData(50, "Luna")]
        public void GenerateAName(int id, string expectedName)
        {
            var name = _moonBuilder.GetName(id);

            Assert.Equal(expectedName, name);
        }

        [Theory]
        [InlineData(0, 26545.23)]
        [InlineData(2, 32768.89)]
        [InlineData(50, 26545.23)]
        public void GenerateAMass(int id, double expectedMass)
        {
            var mass = _moonBuilder.GetMass(id);

            Assert.Equal(expectedMass, mass);
        }

        [Theory]
        [InlineData(0, 56782.23)]
        [InlineData(2, 5667.89)]
        [InlineData(50, 56782.23)]
        public void GenerateADiameter(int id, double expectedDiameter)
        {
            var diameter = _moonBuilder.GetDiameter(id);

            Assert.Equal(expectedDiameter, diameter);
        }
    }
}