using System.Collections.Generic;
using StarTrek.Contracts.World;

namespace StarTrek.Contracts
{
    public interface IMapGenerator
    {
        IGalaxyWorldMap GenerateGalaxyMap();
        IEnumerable<IStarSystem> GenerateGalaxyStarSystems(int amount, IStarSystemGenerator starSystemGenerator);
        IEnumerable<IStarSystem> GenerateStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetGenerator planetGenerator);
        IEnumerable<IStarSystem> GeneratePlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonGenerator moonGenerator);
        void DistributeStarSystems();
    }
}