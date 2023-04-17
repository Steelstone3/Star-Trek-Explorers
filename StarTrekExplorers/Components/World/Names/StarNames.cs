using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.World.Names
{
    public class StarNames : IStarNames
    {
        private readonly string[] starNames = new string[]
        {
            "Sol",
            "Betelgeuse",
            "Risa",
            "Vulcan",
        };

        public string GetName(int seed)
        {
            int index = new RandomGeneration().GetRandomInRange(seed, 0, starNames.Length - 1);
            return starNames[index];
        }
    }
}