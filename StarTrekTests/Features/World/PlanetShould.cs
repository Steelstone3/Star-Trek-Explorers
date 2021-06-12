using Moq;
using StarTrek.Contracts.World;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class PlanetShould
    {
        [Fact]
        public void GenerateAPlanetFromAnId()
        {
            var planetGeneratorMock = GeneratePlanetGeneratorMock();
            var planet = new Planet(0, planetGeneratorMock.Object);

            planetGeneratorMock.Verify(x => x.GetAtmoshere(0));
            planetGeneratorMock.Verify(x => x.GetName(0));
            planetGeneratorMock.Verify(x => x.GetDiameter(0));
            planetGeneratorMock.Verify(x => x.GetMass(0));
        }

        private Mock<IPlanetGenerator> GeneratePlanetGeneratorMock()
        {
            var planetGeneratorMock = new Mock<IPlanetGenerator>();
            planetGeneratorMock.Setup(x => x.GetAtmoshere(0)).Returns("Atmosphere");
            planetGeneratorMock.Setup(x => x.GetName(0)).Returns("Earth");
            planetGeneratorMock.Setup(x => x.GetDiameter(0)).Returns(200);
            planetGeneratorMock.Setup(x => x.GetMass(0)).Returns(100);

            return planetGeneratorMock;
        }
    }
}