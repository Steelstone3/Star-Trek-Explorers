using Moq;
using StarTrek.Contracts.World.Builders;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class PlanetShould
    {
        [Fact]
        public void GenerateAPlanetFromAnId()
        {
            var planetBuilderMock = GeneratePlanetBuilderMock();
            var planet = new Planet(0, planetBuilderMock.Object);

            planetBuilderMock.Verify(x => x.GetAtmoshere(0));
            planetBuilderMock.Verify(x => x.GetName(0));
            planetBuilderMock.Verify(x => x.GetDiameter(0));
            planetBuilderMock.Verify(x => x.GetMass(0));
        }

        private Mock<IPlanetBuilder> GeneratePlanetBuilderMock()
        {
            var planetBuilderMock = new Mock<IPlanetBuilder>();
            planetBuilderMock.Setup(x => x.GetAtmoshere(0)).Returns("Atmosphere");
            planetBuilderMock.Setup(x => x.GetName(0)).Returns("Earth");
            planetBuilderMock.Setup(x => x.GetDiameter(0)).Returns(200);
            planetBuilderMock.Setup(x => x.GetMass(0)).Returns(100);

            return planetBuilderMock;
        }
    }
}