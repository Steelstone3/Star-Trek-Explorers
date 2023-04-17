using System.Collections.Generic;
using StarTrekExplorers.Components.World.Names;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class Star : IStar
    {
        public Star()
        {
            RandomGeneration rng = new();
            Name = new StarNames().GetName(rng.GetSeed());
            StarClass = new StarClasses().GetStarClass(rng.GetSeed());
        }

        public string Name { get; }
        public string StarClass { get; }
        public IEnumerable<IPlanet> Planets { get; } = new PlanetGeneration().GeneratePlanets();
    }
}