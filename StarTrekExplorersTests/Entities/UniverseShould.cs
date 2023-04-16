using StarTrekExplorers.Entities.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class UniverseShould
    {
        private readonly IUniverse universe = new Universe();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotEmpty(universe.Stars);
        }
    }
}