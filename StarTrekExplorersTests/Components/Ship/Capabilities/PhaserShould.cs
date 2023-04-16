using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Capabilities
{
    public class PhaserShould
    {
        private readonly IDamageDealer phaser = new Phaser();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.Equal(10, phaser.Maximum);
            Assert.Equal(5, phaser.Minimum);
        }

        [Theory]
        [InlineData(1234, 6)]
        [InlineData(4321, 5)]
        [InlineData(3333, 9)]
        public void DealDamage(int seed, int expectedDamage)
        {
            // When
            int damage = phaser.DealDamage(seed);

            // Then
            Assert.Equal(expectedDamage, damage);
        }
    }
}