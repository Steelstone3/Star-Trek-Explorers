using Moq;
using StarTrek.Contracts.World.Builders;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features.World
{
    public class MoonShould
    {
        [Fact]
        public void GenerateAMoonFromAnId()
        {
            var moonGeneratorMock = GenerateMoonBuilderMock();
            var planet = new Moon(0, moonGeneratorMock.Object);

            moonGeneratorMock.Verify(x => x.GetName(0));
            moonGeneratorMock.Verify(x => x.GetDiameter(0));
            moonGeneratorMock.Verify(x => x.GetMass(0));
        }

        private Mock<IMoonBuilder> GenerateMoonBuilderMock()
        {
            var moonBuilderMock = new Mock<IMoonBuilder>();
            moonBuilderMock.Setup(x => x.GetName(0)).Returns("Moon");
            moonBuilderMock.Setup(x => x.GetDiameter(0)).Returns(200);
            moonBuilderMock.Setup(x => x.GetMass(0)).Returns(100);

            return moonBuilderMock;
        }
    }
}