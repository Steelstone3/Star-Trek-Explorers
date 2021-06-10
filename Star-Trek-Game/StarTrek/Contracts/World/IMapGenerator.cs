using System.Collections.Generic;
using StarTrek.World;
using StarTrekTests.Features;

namespace StarTrek.Contracts
{
    public interface IMapGenerator
    {
        GalaxyWorldMap GenerateGalaxyMap();
        List<IStarSystems> GeneratePopulatedGalaxyStarSystems();
    }
}