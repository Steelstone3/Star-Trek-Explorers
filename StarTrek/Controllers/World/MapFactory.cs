using System;
using System.Collections.Generic;
using System.Linq;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.World;
using StarTrek.World.CelestialObjects;

namespace StarTrek.Controllers.World
{
    public class MapFactory : IMapFactory
    {
        public IGalaxyWorldMap BuildInitialGalaxyMap()
        {
            var galaxyWorldMap = new GalaxyWorldMap();
            galaxyWorldMap.World = new MapFactoryHelper().GalaxyMap;

            return galaxyWorldMap;
        }

        public IEnumerable<IStarSystem> BuildGalaxyStarSystems(int amount, IStarSystemBuilder starSystemGenerator, IEnumerable<IStarSystem> starSystems)
        {
            //var starSystems = new List<IStarSystem>();
            var starSystemsList = starSystems.ToList();

            for (int i = 0; i < amount; i++)
            {
                var starSystem = new StarSystem(new Random().Next(0, 5), starSystemGenerator, starSystemsList);
                starSystemsList.Add(starSystem);
            }

            return starSystemsList;
        }

        public IEnumerable<IStarSystem> BuildStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetBuilder planetGenerator)
        {
            var planets = new List<IPlanet>();

            foreach (var starSystem in starSystems)
            {
                for (int i = 0; i < new Random().Next(1, 10); i++)
                {
                    planets.Add(new Planet(new Random().Next(0, 5), planetGenerator));
                }

                starSystem.Planets = planets;
            }

            return starSystems;
        }

        public IEnumerable<IStarSystem> BuildPlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonBuilder moonGenerator)
        {
            var moons = new List<IMoon>();

            foreach (var starSystem in starSystems)
            {
                foreach (var planet in starSystem.Planets)
                {
                    for (int i = 0; i < new Random().Next(1, 10); i++)
                    {
                        moons.Add(new Moon(new Random().Next(0, 5), moonGenerator));
                    }

                    planet.Moons = moons;
                }

            }

            return starSystems;
        }
    }
}