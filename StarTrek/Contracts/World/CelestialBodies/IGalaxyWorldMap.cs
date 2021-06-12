using System.Collections.Generic;

namespace StarTrek.Contracts.World.CelestialBodies
{
    public interface IGalaxyWorldMap
    {
        char[,] World { get; set; }
        IEnumerable<IStarSystem> StarSystems { get; set; }
    }
}