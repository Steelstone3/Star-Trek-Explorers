using System.Collections.Generic;

namespace StarTrek.Contracts
{
    public interface IStarSystem
    {
        IEnumerable<IPlanet> Planets { get; set; }
    }
}