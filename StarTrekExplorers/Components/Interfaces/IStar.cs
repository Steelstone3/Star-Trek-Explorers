using System.Collections.Generic;

namespace StarTrekExplorersTests.Entities
{
    public interface IStar
    {
        string Name { get; }
        string Class { get; }
        IEnumerable<IPlanet> Planets { get; }
    }
}