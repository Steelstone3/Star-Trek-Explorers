using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public class IdentificationShould
    {
        const int SEED = 1234;

        [Fact]
        public void ConstructFederation()
        {
            // Given
            const Faction faction = Faction.Federation;
            IIdentification identification = new Identification(SEED, faction);

            // Then
            Assert.Equal(faction, identification.Faction);
            Assert.Equal("USS-45917", identification.SerialNumber);
            Assert.Equal("Excelsio", identification.Name);
            Assert.Equal("Saber", identification.Class);
        }

        [Fact]
        public void ConstructKlingon()
        {
            // Given
            const Faction faction = Faction.KlingonEmpire;
            IIdentification identification = new Identification(SEED, faction);

            // Then
            Assert.Equal(faction, identification.Faction);
            Assert.Equal("IKS-45917", identification.SerialNumber);
            Assert.Equal("K't'inga", identification.Name);
            Assert.Equal("Lotl'eh", identification.Class);
        }
    }
}