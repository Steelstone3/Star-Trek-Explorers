using System.Collections.Generic;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.Contracts.World
{
    public interface IMapFactory
    {
        IGalaxyWorldMap BuildInitialGalaxyMap();
        IEnumerable<IStarSystem> BuildGalaxyStarSystems(int amount, IStarSystemBuilder starSystemGenerator, IEnumerable<IStarSystem> starSystems);
        IEnumerable<IStarSystem> BuildStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetBuilder planetGenerator);
        IEnumerable<IStarSystem> BuildPlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonBuilder moonGenerator);
    }
}