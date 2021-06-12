using System.Collections.Generic;

namespace StarTrek.Contracts.World.CelestialBodies
{
    public interface IPlanet
    {
        IEnumerable<IMoon> Moons { get; set; }
        string Name { get; }
        string Atmosphere { get; }
        double Mass { get; }
        double Diameter { get; }
    }
}