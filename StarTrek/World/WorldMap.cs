using System.Collections.Generic;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World
{
    public class WorldMap
    {
        public WorldMap(IMapFactory mapGenerator, 
        IStarSystemBuilder starSystemGenerator, 
        IEnumerable<IStarSystem> starSystems,
        IPlanetBuilder planetGenerator, 
        IMoonBuilder moonGenerator)
        {
            Galaxy = mapGenerator.BuildInitialGalaxyMap();
            Galaxy.StarSystems = mapGenerator.BuildGalaxyStarSystems(250, starSystemGenerator, starSystems);
            Galaxy.StarSystems = mapGenerator.BuildStarSystemPlanets(Galaxy.StarSystems, planetGenerator);
            Galaxy.StarSystems = mapGenerator.BuildPlanetMoons(Galaxy.StarSystems, moonGenerator);
        }

        public IGalaxyWorldMap Galaxy { get; private set; }
    }
}