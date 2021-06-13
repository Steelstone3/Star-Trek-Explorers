using System.Collections.Generic;
using System.Linq;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.Controllers.World;
using StarTrek.Controllers.World.Builders;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldBuilderShould
    {
        private List<IStarSystem> _starSystems;

        private IMapFactory _mapFactory = new MapFactory();

        public WorldBuilderShould()
        {
            _starSystems = new List<IStarSystem>()
            {
                new StarSystem("Bob", "Red Dwarf", 3000, 200000, 2, 3),
                new StarSystem("Dave", "Yellow", 7000, 10000, 1, 4),
            };
        }

        //Generate a galaxy (world map)
        [Fact]
        public void GenerateAGalaxyMap()
        {
            //Act
            var map = _mapFactory.GenerateInitialGalaxyMap();

            //Assert
            Assert.NotNull(map);
            Assert.NotNull(map.World);
            Assert.NotEmpty(map.World);
        }

        //Populate the galaxy with randomly generated persistant star systems
        [Fact]
        public void GenerateStarSystems()
        {
            //Act
            var starSystems = _mapFactory.GenerateGalaxyStarSystems(10, new StarSystemBuilder());

            //Assert
            Assert.NotNull(starSystems);
            Assert.NotEmpty(starSystems);
            Assert.Equal(10, starSystems.ToList().Count);
        }

        //Populate star systems with randomly generated persistant planets
        [Fact]
        public void GeneratePlanets()
        {
            //Act
            var starSystems = _mapFactory.GenerateStarSystemPlanets(_starSystems, new PlanetBuilder());

            //Assert
            foreach (var starSystem in starSystems)
            {
                Assert.NotNull(starSystem.Planets);
                Assert.NotEmpty(starSystem.Planets);
            }
        }

        //Populate planets with randomly generated persistant moons
        [Fact]
        public void GenerateMoons()
        {
            //Act
            var starSystems = _mapFactory.GeneratePlanetMoons(_starSystems, new MoonBuilder());

            //Assert
            foreach (var starSystem in starSystems)
            {
                foreach (var planet in starSystem.Planets)
                {
                    Assert.NotNull(planet.Moons);
                    Assert.NotEmpty(planet.Moons);
                }
            }
        }
        
        //Distribute the star systems across the galaxy (world map)
        [Fact(Skip="Implement this functionality")]
        public void DistributeStarSystemsAcrossTheGalaxy()
        {

        }
    }
}