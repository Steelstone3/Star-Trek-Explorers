using System.Collections.Generic;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class StarGeneration : IStarGeneration
    {
        public IEnumerable<IStar> GenerateStars()
        {
            List<IStar> stars = new();

            RandomGeneration randomGeneration = new();
            int amount = randomGeneration.GetRandomInRange(randomGeneration.GetSeed(), 100, 500);
            AddStars(stars, amount);

            return stars;
        }

        private static void AddStars(List<IStar> stars, int amount)
        {
            for (int i = 0; i < amount; i++)
            {
                IStar star = new Star();
                stars.Add(star);
            }
        }
    }
}