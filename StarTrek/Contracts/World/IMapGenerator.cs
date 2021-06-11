using System.Collections.Generic;
using StarTrek.Contracts.World;
using StarTrek.World;
using StarTrek.World.CelestialObjects;

namespace StarTrek.Contracts
{
    public interface IMapGenerator
    {
        GalaxyWorldMap GenerateGalaxyMap();
        IEnumerable<IStarSystem> GenerateGalaxyStarSystems(int amount, IStarSystemGenerator starSystemGenerator);
        IEnumerable<IStarSystem> GenerateStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetGenerator planetGenerator);
        IEnumerable<IStarSystem> GeneratePlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonGenerator moonGenerator);
    }
}