using Moq;
using StarTrek.Contracts.World;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class MoonShould
    {
        [Fact]
        public void GenerateAMoonFromAnId()
        {
            var moonGeneratorMock = GenerateMoonGeneratorMock();
            var planet = new Moon(0, moonGeneratorMock.Object);

            moonGeneratorMock.Verify(x => x.GetName(0));
            moonGeneratorMock.Verify(x => x.GetDiameter(0));
            moonGeneratorMock.Verify(x => x.GetMass(0));
        }

        private Mock<IMoonGenerator> GenerateMoonGeneratorMock()
        {
            var moonGeneratorMock = new Mock<IMoonGenerator>();
            moonGeneratorMock.Setup(x => x.GetName(0)).Returns("Moon");
            moonGeneratorMock.Setup(x => x.GetDiameter(0)).Returns(200);
            moonGeneratorMock.Setup(x => x.GetMass(0)).Returns(100);

            return moonGeneratorMock;
        }
    }
}