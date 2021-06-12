using StarTrek.Contracts;
using StarTrek.Contracts.World;

namespace StarTrek.World
{
    public class WorldMap
    {
        public WorldMap(IMapGenerator mapGenerator, 
        IStarSystemGenerator starSystemGenerator, 
        IPlanetGenerator planetGenerator, 
        IMoonGenerator moonGenerator)
        {
            Galaxy = mapGenerator.GenerateGalaxyMap();
            Galaxy.StarSystems = mapGenerator.GenerateGalaxyStarSystems(10, starSystemGenerator);
            Galaxy.StarSystems = mapGenerator.GenerateStarSystemPlanets(Galaxy.StarSystems, planetGenerator);
            Galaxy.StarSystems = mapGenerator.GeneratePlanetMoons(Galaxy.StarSystems, moonGenerator);
            mapGenerator.DistributeStarSystems();
        }

        public IGalaxyWorldMap Galaxy { get; private set; }
    }
}