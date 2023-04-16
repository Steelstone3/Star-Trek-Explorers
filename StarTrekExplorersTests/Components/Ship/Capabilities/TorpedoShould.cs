using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Capabilities
{
    public class TorpedoShould
    {
        private readonly IDamageDealer torpedo = new Torpedo();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.Equal(10, torpedo.Maximum);
            Assert.Equal(5, torpedo.Minimum);
        }

        [Theory]
        [InlineData(1234, 6)]
        [InlineData(4321, 5)]
        [InlineData(3333, 9)]
        public void DealDamage(int seed, int expectedDamage)
        {
            // When
            int damage = torpedo.DealDamage(seed);

            // Then
            Assert.Equal(expectedDamage, damage);
        }
    }
}