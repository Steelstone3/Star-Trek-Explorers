using StarTrek.Controllers;
using Xunit;

namespace StarTrekTests.Features
{
    public class MoonGeneratorShould
    {
        private MoonGenerator _moonGenerator = new MoonGenerator();

        [Theory]
        [InlineData(0, "Luna")]
        [InlineData(1, "Titan")]
        [InlineData(50, "Luna")]
        public void GenerateAName(int id, string expectedName)
        {
            var name = _moonGenerator.GetName(id);

            Assert.Equal(expectedName, name);
        }

        [Theory]
        [InlineData(0, 26545.23)]
        [InlineData(2, 32768.89)]
        [InlineData(50, 26545.23)]
        public void GenerateAMass(int id, double expectedMass)
        {
            var mass = _moonGenerator.GetMass(id);

            Assert.Equal(expectedMass, mass);
        }

        [Theory]
        [InlineData(0, 56782.23)]
        [InlineData(2, 5667.89)]
        [InlineData(50, 56782.23)]
        public void GenerateADiameter(int id, double expectedDiameter)
        {
            var diameter = _moonGenerator.GetDiameter(id);

            Assert.Equal(expectedDiameter, diameter);
        }
    }
}