using StarTrek.Controllers;
using Xunit;

namespace StarTrekTests.Features
{
    public class PlanetGeneratorShould
    {
        private PlanetGenerator _planetGenerator = new PlanetGenerator();

        [Theory]
        [InlineData(0, "Earth")]
        [InlineData(2, "Venus")]
        [InlineData(50, "Earth")]
        public void GenerateAName(int id, string expectedName)
        {
            var name = _planetGenerator.GetName(id);

            Assert.Equal(expectedName, name);
        }

        [Theory]
        [InlineData(0, "0% O2, 0% N, 0% Other")]
        [InlineData(2, "100% O2, 0% N, 0% Other")]
        [InlineData(50, "0% O2, 0% N, 0% Other")]
        public void GenerateAnAtmosphere(int id, string expectedAtmosphere)
        {
            var atmosphere = _planetGenerator.GetAtmoshere(id);

            Assert.Equal(expectedAtmosphere, atmosphere);
        }

        [Theory]
        [InlineData(0, 26545.23)]
        [InlineData(2, 32768.89)]
        [InlineData(50, 26545.23)]
        public void GenerateAMass(int id, double expectedMass)
        {
            var mass = _planetGenerator.GetMass(id);

            Assert.Equal(expectedMass, mass);
        }

        [Theory]
        [InlineData(0, 56782.23)]
        [InlineData(2, 5667.89)]
        [InlineData(50, 56782.23)]
        public void GenerateADiameter(int id, double expectedDiameter)
        {
            var diameter = _planetGenerator.GetDiameter(id);

            Assert.Equal(expectedDiameter, diameter);
        }
    }
}