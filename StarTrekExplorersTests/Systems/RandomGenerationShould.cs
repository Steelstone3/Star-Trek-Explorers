using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class RandomGenerationShould
    {
        private readonly IRandomGeneration randomGeneration = new RandomGeneration();

        [Theory]
        [InlineData(1234, 20)]
        [InlineData(4321, 7)]
        [InlineData(1221, 30)]
        public void GetARandomValueInRange(int seed, int expectedValue)
        {
            // When
            int value = randomGeneration.GetRandomInRange(seed, 1, 50);

            // Then
            Assert.Equal(expectedValue, value);
        }
    }
}