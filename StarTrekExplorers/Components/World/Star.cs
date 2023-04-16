using System.Collections.Generic;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Systems
{
    public class Star : IStar
    {
        public string Name { get; } = "";
        public string Class { get; } = "";
        public IEnumerable<IPlanet> Planets { get; } = new PlanetGeneration().GeneratePlanets();
    }
}