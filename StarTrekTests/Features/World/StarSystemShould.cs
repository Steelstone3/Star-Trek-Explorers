using System;
using System.Collections.Generic;
using Moq;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features.World
{
    public class StarSystemShould
    {
        private Mock<IEnumerable<IStarSystem>> _starSystemsMock = new Mock<IEnumerable<IStarSystem>>();

        [Fact]
        public void GenerateAStarSystemFromAnId()
        {
            var starSystemBuilderMock = GenerateStarSystemBuilderMock();

            var planet = new StarSystem(0, starSystemBuilderMock.Object, _starSystemsMock.Object);

            starSystemBuilderMock.Verify(x => x.GetName(0));
            starSystemBuilderMock.Verify(x => x.GetType(0));
            starSystemBuilderMock.Verify(x => x.GetDiameter(0));
            starSystemBuilderMock.Verify(x => x.GetMass(0));
            starSystemBuilderMock.Verify(x => x.SetUniqueLocation(0, 0, _starSystemsMock.Object));
        }

        private Mock<IStarSystemBuilder> GenerateStarSystemBuilderMock()
        {
            var starSystemBuilderMock = new Mock<IStarSystemBuilder>();
            starSystemBuilderMock.Setup(x => x.GetName(0)).Returns("Moon");
            starSystemBuilderMock.Setup(x => x.GetType(0)).Returns("Yellow");
            starSystemBuilderMock.Setup(x => x.GetDiameter(0)).Returns(200);
            starSystemBuilderMock.Setup(x => x.GetMass(0)).Returns(100);
            starSystemBuilderMock.Setup(x => x.SetUniqueLocation(0, 0, _starSystemsMock.Object)).Returns(new Tuple<int, int>(0, 0));

            return starSystemBuilderMock;
        }
    }
}