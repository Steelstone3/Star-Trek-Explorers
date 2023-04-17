using System.Collections.Generic;

namespace StarTrekExplorersTests.Entities
{
    public interface IStar
    {
        string Name { get; }
        string StarClass { get; }
        IEnumerable<IPlanet> Planets { get; }
    }
}