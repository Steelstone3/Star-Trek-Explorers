using System.Collections.Generic;
using System.Linq;
using StarTrek.Contracts;
using StarTrek.Controllers;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldGeneratorShould
    {
        private List<IStarSystem> _starSystems;

        private IMapGenerator _mapGenerator = new MapGenerator();

        public WorldGeneratorShould()
        {
            _starSystems = new List<IStarSystem>()
            {
                new StarSystem("Bob", "Red Dwarf", 3000, 200000),
                new StarSystem("Dave", "Yellow", 7000, 10000),
            };
        }

        //Generate a galaxy (world map)
        [Fact]
        public void GenerateAGalaxyMap()
        {
            //Act
            var map = _mapGenerator.GenerateGalaxyMap();

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
            var starSystems = _mapGenerator.GenerateGalaxyStarSystems(10, new StarSystemGenerator());

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
            var starSystems = _mapGenerator.GenerateStarSystemPlanets(_starSystems, new PlanetGenerator());

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
            var starSystems = _mapGenerator.GeneratePlanetMoons(_starSystems, new MoonGenerator());

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
    }
}