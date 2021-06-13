using Moq;
using StarTrek.Contracts;
using StarTrek.Contracts.World.Builders;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class StarSystemShould
    {
        [Fact]
        public void GenerateAStarSystemFromAnId()
        {
            var starSystemBuilderMock = GenerateStarSystemBuilderMock();
            var planet = new StarSystem(0, starSystemBuilderMock.Object);

            starSystemBuilderMock.Verify(x => x.GetName(0));
            starSystemBuilderMock.Verify(x => x.GetType(0));
            starSystemBuilderMock.Verify(x => x.GetDiameter(0));
            starSystemBuilderMock.Verify(x => x.GetMass(0));
        }

        private Mock<IStarSystemBuilder> GenerateStarSystemBuilderMock()
        {
            var starSystemBuilderMock = new Mock<IStarSystemBuilder>();
            starSystemBuilderMock.Setup(x => x.GetName(0)).Returns("Moon");
            starSystemBuilderMock.Setup(x => x.GetType(0)).Returns("Yellow");
            starSystemBuilderMock.Setup(x => x.GetDiameter(0)).Returns(200);
            starSystemBuilderMock.Setup(x => x.GetMass(0)).Returns(100);

            return starSystemBuilderMock;
        }
    }
}