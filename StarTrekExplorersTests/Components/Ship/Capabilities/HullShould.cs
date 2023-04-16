using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.Ship.Capabilities
{
    public class HullShould
    {
        private readonly IDefense hull = new Hull();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.Equal(100, hull.Current);
            Assert.Equal(100, hull.Maximum);
            Assert.Equal(5, hull.RepairRate);
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
            hull.TakeDamage(damage);

            // Then
            Assert.Equal(remainingShield, hull.Current);
            Assert.Equal(100, hull.Maximum);
            Assert.Equal(5, hull.RepairRate);
        }
    }
}