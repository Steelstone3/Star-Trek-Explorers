using System.Collections.Generic;
using StarTrek.Contracts;
using StarTrek.Contracts.World;

namespace StarTrek.World
{
    public class GalaxyWorldMap : IGalaxyWorldMap
    {
        public char[,] World { get; set; }
        public IEnumerable<IStarSystem> StarSystems { get; set; }
    }
}