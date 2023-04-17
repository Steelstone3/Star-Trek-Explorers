using System.Security.Cryptography;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.World.Names
{
    public class PlanetNames : IPlanetNames
    {
        private readonly string[] planetNames = new string[]
        {
            "Earth",
            "Mars",
            "Venus",
            "Mercury",
            "Pluto"
        };

        public string GetName(int seed)
        {
            int index = new RandomGeneration().GetRandomInRange(seed, 0, planetNames.Length - 1);
            return planetNames[index];
        }
    }
}