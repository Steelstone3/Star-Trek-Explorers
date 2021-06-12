using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World
{
    public class WorldMap
    {
        public WorldMap(IMapFactory mapGenerator, 
        IStarSystemBuilder starSystemGenerator, 
        IPlanetBuilder planetGenerator, 
        IMoonBuilder moonGenerator)
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