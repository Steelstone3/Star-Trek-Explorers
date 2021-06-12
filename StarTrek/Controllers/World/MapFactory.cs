using System;
using System.Collections.Generic;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.World;
using StarTrek.World.CelestialObjects;

namespace StarTrek.Controllers.World
{
    public class MapFactory : IMapFactory
    {
        public IGalaxyWorldMap GenerateGalaxyMap()
        {
            var galaxyWorldMap = new GalaxyWorldMap();

            galaxyWorldMap.World = new char[,]
            {
                {'*', '*', '*', '*', '*', '*', '*', '*', '*', '*'},
                {'*', '*', '*', '*', '*', '*', '*', '*', '*', '*'},
            };

            return galaxyWorldMap;
        }

        public IEnumerable<IStarSystem> GenerateGalaxyStarSystems(int amount, IStarSystemBuilder starSystemGenerator)
        {
            var starSystems = new List<IStarSystem>();

            for (int i = 0; i < amount; i++)
            {
                starSystems.Add(new StarSystem(new Random().Next(0, 5), starSystemGenerator));
            }

            return starSystems;
        }

        public IEnumerable<IStarSystem> GenerateStarSystemPlanets(IEnumerable<IStarSystem> starSystems, IPlanetBuilder planetGenerator)
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

        public IEnumerable<IStarSystem> GeneratePlanetMoons(IEnumerable<IStarSystem> starSystems, IMoonBuilder moonGenerator)
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

        //TODO AH think about how you want this to be done
        public void DistributeStarSystems()
        {
            Console.WriteLine("PLEASE IMPLEMENT ME!!!");
        }
    }
}