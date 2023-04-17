using StarTrekExplorers.Systems;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Components.World
{
    public class StarShould
    {
        private readonly IStar star = new Star();

        [Fact]
        public void TestName()
        {
            // Then
            Assert.NotNull(star.Name);
            Assert.NotNull(star.StarClass);
            Assert.NotEmpty(star.Planets);
        }
    }
}