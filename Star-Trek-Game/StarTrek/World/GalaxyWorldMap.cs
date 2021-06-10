using System.Collections.Generic;
using StarTrek.Contracts;

namespace StarTrek.World
{
    public class GalaxyWorldMap
    {
        public char[,] World { get; set; }
        public List<IStarSystems> StarSystems { get; set; }
    }
}