using System.Collections.Generic;
using StarTrek.Contracts;

namespace StarTrek.World
{
    public class GalaxyWorldMap
    {
        public char[,] World { get; set; }
        public IEnumerable<IStarSystem> StarSystems { get; set; }
    }
}