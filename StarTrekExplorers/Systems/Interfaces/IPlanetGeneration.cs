using System.Collections.Generic;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface IPlanetGeneration
    {
        IEnumerable<IPlanet> GeneratePlanets();
    }
}