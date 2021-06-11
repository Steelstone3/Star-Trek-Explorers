using System;
using System.Collections.Generic;
using StarTrek.Contracts;
using StarTrek.Contracts.World;
using StarTrek.World;
using StarTrek.World.CelestialObjects;

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

        public IEnumerable<IStarSystem> GenerateGalaxyStarSystems(int amount, IStarSystemGenerator starSystemGenerator)
        {
            var starSystems = new List<IStarSystem>();

            for (int i = 0; i < amount; i++)
            {
                starSystems.Add(new StarSystem(new Random().Next(0, 5), starSystemGenerator));
            }

            return starSystems;
        }

        public IEnumerable<IStarSystem> GenerateStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetGenerator planetGenerator)
        {
            var planets = new List<IPlanet>();

            foreach (var starSystem in starSystems)
            {
                for (int i = 0; i < new Random().Next(1, 10); i++)
                {
                    planets.Add(new Planet(new Random().Next(0, 5), planetGenerator));
                }

                starSystem.Planets.AddRange(planets);
            }

            return starSystems;
        }

        public IEnumerable<IStarSystem> GeneratePlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonGenerator moonGenerator)
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

                    planet.Moons.AddRange(moons);
                }

            }

            return starSystems;
        }
    }
}