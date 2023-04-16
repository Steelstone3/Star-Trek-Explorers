using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Systems.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class SerialNumberGenerationShould
    {
        private readonly ISerialNumberGeneration serialNumberGeneration = new SerialNumberGeneration();

        [Theory]
        [InlineData(1234, "USS-45917")]
        [InlineData(4321, "USS-21342")]
        public void GenerateFederationSerialNumber(int seed, string expectedSerialNumber)
        {
            // When
            string serialNumber = serialNumberGeneration.GenerateSerialNumber(seed, Faction.Federation);

            // Then
            Assert.Equal(expectedSerialNumber, serialNumber);
        }

        [Theory]
        [InlineData(1234, "IKS-45917")]
        [InlineData(4321, "IKS-21342")]
        public void GenerateKlingonSerialNumber(int seed, string expectedSerialNumber)
        {
            // When
            string serialNumber = serialNumberGeneration.GenerateSerialNumber(seed, Faction.KlingonEmpire);

            // Then
            Assert.Equal(expectedSerialNumber, serialNumber);
        }
    }
}