using Moq;
using StarTrek.Contracts;
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
            var planetGenerator = new Mock<IPlanetGenerator>();
            planetGenerator.Setup(x => x.GetAtmoshere(0)).Returns("Atmosphere");
            planetGenerator.Setup(x => x.GetName(0)).Returns("Earth");
            planetGenerator.Setup(x => x.GetDiameter(0)).Returns(200);
            planetGenerator.Setup(x => x.GetMass(0)).Returns(100);

            return planetGenerator;
        }
    }
}