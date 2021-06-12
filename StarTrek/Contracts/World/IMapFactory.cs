using System.Collections.Generic;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.Contracts.World
{
    public interface IMapFactory
    {
        IGalaxyWorldMap GenerateGalaxyMap();
        IEnumerable<IStarSystem> GenerateGalaxyStarSystems(int amount, IStarSystemBuilder starSystemGenerator);
        IEnumerable<IStarSystem> GenerateStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetBuilder planetGenerator);
        IEnumerable<IStarSystem> GeneratePlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonBuilder moonGenerator);
        void DistributeStarSystems();
    }
}