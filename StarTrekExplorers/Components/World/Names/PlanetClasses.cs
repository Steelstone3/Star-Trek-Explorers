using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.World.Names
{
    public class PlanetClasses : IPlanetClasses
    {
        private readonly string[] planetClasses = new string[]
        {
            "Class A",
            "Class B",
            "Class M",
        };

        public string GetPlanetClass(int seed)
        {
            int index = new RandomGeneration().GetRandomInRange(seed, 0, planetClasses.Length - 1);
            return planetClasses[index];
        }
    }
}