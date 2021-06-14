using System;
using System.Collections.Generic;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.Controllers.World.Builders;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features.World
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

        [Theory]
        [InlineData(0, 0, 0, 0, 0, 1, 1, 1, 1, 0)]
        [InlineData(0, 0, 3, 0, 0, 0, 2, 0, 1, 0)]
        [InlineData(2, 0, 3, 0, 0, 0, 2, 0, 1, 0)]
        [InlineData(1, 2, 1, 2, 0, 0, 0, 2, 0, 1)]
        [InlineData(1, 1, 3, 1, 1, 1, 1, 2, 2, 1)]
        public void GenerateAUniqueLocation(int coordinateX, int coordinateY, int expectedXCoordinate, int expectedYCoordinate, int unexpectedXCoordinate1, int unexpectedYCoordinate1, int unexpectedXCoordinate2, int unexpectedYCoordinate2, int unexpectedXCoordinate3, int unexpectedYCoordinate3)
        {
            var expectedCoordinateSet = new Tuple<int, int>(expectedXCoordinate, expectedYCoordinate);
            var coordinateSet1 = new Tuple<int, int>(unexpectedXCoordinate1, unexpectedYCoordinate1);
            var coordinateSet2 = new Tuple<int, int>(unexpectedXCoordinate2, unexpectedYCoordinate2);
            var coordinateSet3 = new Tuple<int, int>(unexpectedXCoordinate3, unexpectedYCoordinate3);

            var starSystems = new List<IStarSystem>()
            {
                new StarSystem("Sun", "Yellow", 1000, 2000, coordinateSet1.Item1, coordinateSet1.Item2),
                new StarSystem("Redius", "Red Giant", 19000, 3200, coordinateSet2.Item1, coordinateSet2.Item2),
                new StarSystem("Jeffos", "White Giant", 109900, 70000, coordinateSet3.Item1, coordinateSet3.Item2),
            };

            var starSystemLocation = _starSystemBuilder.SetUniqueLocation(coordinateX, coordinateY, starSystems);

            Assert.Equal(expectedCoordinateSet, starSystemLocation);
            Assert.NotEqual(coordinateSet1, starSystemLocation);
            Assert.NotEqual(coordinateSet2, starSystemLocation);
            Assert.NotEqual(coordinateSet3, starSystemLocation);
        }
    }
}