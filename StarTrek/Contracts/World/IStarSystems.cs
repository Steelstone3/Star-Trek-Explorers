using System.Collections.Generic;

namespace StarTrek.Contracts
{
    public interface IStarSystem
    {
        List<IPlanet> Planets { get; set; }
    }
}