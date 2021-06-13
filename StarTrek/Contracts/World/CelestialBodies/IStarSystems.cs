using System.Collections.Generic;

namespace StarTrek.Contracts.World.CelestialBodies
{
    public interface IStarSystem
    {
        IEnumerable<IPlanet> Planets { get; set; }

        int CoordinateLocationX { get; set; }
        int CoordinateLocationY { get; set; }
    }
}