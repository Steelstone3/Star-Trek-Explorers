using System;
using StarTrekExplorers.Systems.Interfaces;

namespace StarTrekExplorersTests.Systems
{
    public class RandomGeneration : IRandomGeneration
    {
        public int GetRandomInRange(int seed, int minimum, int maximum)
        {
            Random random = new(seed);
            return random.Next(minimum, maximum);
        }

        public int GetSeed() => new Random().Next();
    }
}