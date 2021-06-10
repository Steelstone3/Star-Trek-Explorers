using System.Collections.Generic;

namespace StarTrek.Contracts
{
    public interface IPlanet
    {
        List<IMoon> Moons { get; set; }
        string Name { get; }
        string Atmosphere { get; }
        double Mass { get; }
        double Diameter { get; }
    }
}