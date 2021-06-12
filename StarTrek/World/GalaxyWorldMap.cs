using System.Collections.Generic;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World
{
    public class GalaxyWorldMap : IGalaxyWorldMap
    {
        public char[,] World { get; set; }
        public IEnumerable<IStarSystem> StarSystems { get; set; }
    }
}