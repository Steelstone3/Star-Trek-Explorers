using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Systems
{
    public class Planet : IPlanet
    {
        public string Name { get; } = "";
        public string Class { get; } = "";
    }
}