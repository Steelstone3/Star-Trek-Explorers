using System.Collections.Generic;
using StarTrek.Contracts.World.Builders;

namespace StarTrek.Contracts.World.CelestialBodies
{
    public interface IStarSystem
    {
        IEnumerable<IPlanet> Planets { get; set; }
         string Name { get; }
         string Type {get;}
         double Mass { get; }
         double Diameter { get; }
        int CoordinateLocationX { get; set; }
        int CoordinateLocationY { get; set; }
    }
}