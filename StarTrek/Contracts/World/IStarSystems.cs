using System.Collections.Generic;

namespace StarTrek.Contracts
{
    public interface IStarSystems
    {
        List<IPlanet> Planets { get; set; }
    }
}