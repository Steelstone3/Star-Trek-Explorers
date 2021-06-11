using Moq;
using StarTrek.Contracts;
using StarTrek.Contracts.World;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class StarSystemShould
    {
        [Fact]
        public void GenerateAStarSystemFromAnId()
        {
            var starSystemGeneratorMock = GenerateMoonGeneratorMock();
            var planet = new StarSystem(0, starSystemGeneratorMock.Object);

            starSystemGeneratorMock.Verify(x => x.GetName(0));
            starSystemGeneratorMock.Verify(x => x.GetType(0));
            starSystemGeneratorMock.Verify(x => x.GetDiameter(0));
            starSystemGeneratorMock.Verify(x => x.GetMass(0));
        }

        private Mock<IStarSystemGenerator> GenerateMoonGeneratorMock()
        {
            var starSystemGeneratorMock = new Mock<IStarSystemGenerator>();
            starSystemGeneratorMock.Setup(x => x.GetName(0)).Returns("Moon");
            starSystemGeneratorMock.Setup(x => x.GetType(0)).Returns("Yellow");
            starSystemGeneratorMock.Setup(x => x.GetDiameter(0)).Returns(200);
            starSystemGeneratorMock.Setup(x => x.GetMass(0)).Returns(100);

            return starSystemGeneratorMock;
        }
    }
}