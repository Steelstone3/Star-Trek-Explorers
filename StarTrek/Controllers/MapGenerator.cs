using System.Collections.Generic;
using StarTrek.Contracts;
using StarTrek.World;
using StarTrekTests.Features;

namespace StarTrek.Controllers
{
    public class MapGenerator : IMapGenerator
    {
        public GalaxyWorldMap GenerateGalaxyMap()
        {
            var galaxyWorldMap = new GalaxyWorldMap();

            galaxyWorldMap.World = new char[,]
            {
                {'*', '*', '*', '*', '*', '*', '*', '*', '*', '*'},
                {'*', '*', '*', '*', '*', '*', '*', '*', '*', '*'},
            };

            return galaxyWorldMap;
        }

        public List<IStarSystems> GeneratePopulatedGalaxyStarSystems()
        {
            return new List<IStarSystems>()
            {
               
            };
        } 

        private List<IMoon> GeneratePlanetMoons()
        {
            return null;
        }

        private List<IPlanet> GenerateStarSystemPlanets()
        {
            return null;
        }

        
    }
}