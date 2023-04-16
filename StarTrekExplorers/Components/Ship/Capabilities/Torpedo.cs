using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Torpedo : IDamageDealer
    {
        public int Maximum { get; } = 10;
        public int Minimum { get; } = 5;

        public int DealDamage(int seed)
        {
            RandomGeneration randomGeneration = new();
            return randomGeneration.GetRandomInRange(seed, Minimum, Maximum);
        }
    }
}