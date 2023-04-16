using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Capabilities
{
    public class ShieldShould
    {
        private readonly IDamageTaker shield = new Shield();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.Equal(100, shield.Current);
            Assert.Equal(100, shield.Maximum);
            Assert.Equal(5, shield.RepairRate);
        }

        [Theory]
        [InlineData(25, 75)]
        [InlineData(50, 50)]
        [InlineData(100, 0)]
        [InlineData(101, 0)]
        [InlineData(200, 0)]
        public void TakeDamage(int damage, int remainingShield)
        {
            // When
            shield.TakeDamage(damage);

            // Then
            Assert.Equal(remainingShield, shield.Current);
            Assert.Equal(100, shield.Maximum);
            Assert.Equal(5, shield.RepairRate);
        }
    }
}