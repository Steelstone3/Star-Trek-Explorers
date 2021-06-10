using StarTrek.Contracts;

namespace StarTrek.World
{
    public class WorldMap
    {
        public WorldMap(IMapGenerator mapGenerator)
        {
            Galaxy = mapGenerator.GenerateGalaxyMap();
            Galaxy.StarSystems = mapGenerator.GeneratePopulatedGalaxyStarSystems();
        }

        public GalaxyWorldMap Galaxy { get; private set; }
    }
}