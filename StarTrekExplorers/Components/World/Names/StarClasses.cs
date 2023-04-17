using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.World.Names
{
    public class StarClasses : IStarClasses
    {
        private readonly string[] starClasses = new string[]
        {
            "Class A",
            "Class B",
            "Class M",
        };

        public string GetStarClass(int seed)
        {
            int index = new RandomGeneration().GetRandomInRange(seed, 0, starClasses.Length - 1);
            return starClasses[index];
        }
    }
}