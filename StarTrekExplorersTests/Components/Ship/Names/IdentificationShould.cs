using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public class IdentificationShould
    {
        [Fact]
        public void ConstructFederation()
        {
            // Given
            const Faction faction = Faction.Federation;
            IIdentification identification = new Identification(1234, faction);

            // Then
            Assert.Equal(faction, identification.Faction);
            Assert.Equal("", identification.SerialNumber);
            Assert.Equal("", identification.Name);
            Assert.Equal("", identification.Class);
        }

        [Fact]
        public void ConstructKlingon()
        {
            // Given
            const Faction faction = Faction.KlingonEmpire;
            IIdentification identification = new Identification(1234, faction);

            // Then
            Assert.Equal(faction, identification.Faction);
            Assert.Equal("", identification.SerialNumber);
            Assert.Equal("", identification.Name);
            Assert.Equal("", identification.Class);
        }
    }
}