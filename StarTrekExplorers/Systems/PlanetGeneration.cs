using System.Collections.Generic;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class PlanetGeneration : IPlanetGeneration
    {
        public IEnumerable<IPlanet> GeneratePlanets()
        {
            List<IPlanet> planets = new();

            RandomGeneration randomGeneration = new();
            int amount = randomGeneration.GetRandomInRange(randomGeneration.GetSeed(), 1, 10);
            AddStars(planets, amount);

            return planets;
        }

        private static void AddStars(List<IPlanet> planets, int amount)
        {
            for (int i = 0; i < amount; i++)
            {
                IPlanet planet = new Planet();
                planets.Add(planet);
            }
        }
    }
}