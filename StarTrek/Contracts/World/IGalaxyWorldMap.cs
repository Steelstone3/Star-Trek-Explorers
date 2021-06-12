using System.Collections.Generic;

namespace StarTrek.Contracts.World
{
    public interface IGalaxyWorldMap
    {
        char[,] World { get; set; }
        IEnumerable<IStarSystem> StarSystems { get; set; }
    }
}