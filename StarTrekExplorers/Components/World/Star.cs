using System.Collections.Generic;
using StarTrekExplorers.Components.World.Names;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Systems
{
    public class Star : IStar
    {
        // TODO Get name/ class
        public string Name { get; } = new StarNames().GetName();
        public string Class { get; } = new StarClasses().GetClass();
        public IEnumerable<IPlanet> Planets { get; } = new PlanetGeneration().GeneratePlanets();
    }
}