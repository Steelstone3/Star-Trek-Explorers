using StarTrekExplorers.Components.World.Names;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class Planet : IPlanet
    {
        public Planet()
        {
            RandomGeneration rng = new();
            Name = new PlanetNames().GetName(rng.GetSeed());
            PlanetClass = new PlanetClasses().GetPlanetClass(rng.GetSeed());
        }

        public string Name { get; }
        public string PlanetClass { get; }
    }
}