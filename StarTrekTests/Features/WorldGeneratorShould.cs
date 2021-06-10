using StarTrek.Contracts;
using StarTrek.Controllers;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldGeneratorShould
    {


        private IMapGenerator _mapGenerator = new MapGenerator();

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

        //Probably should refactor this test
        //Populate the galaxy with randomly generated persistant star systems
        //Populate star systems with randomly generated persistant planets and moons
        [Fact(Skip = "Big messy test")]
        public void GenerateAPopulatedStarSystem()
        {
            //Act
            var starSystems = _mapGenerator.GeneratePopulatedGalaxyStarSystems();

            //Assert
            Assert.NotNull(starSystems);
            Assert.NotEmpty(starSystems);

            foreach (var starSystem in starSystems)
            {
                Assert.NotNull(starSystem.Planets);
                Assert.NotEmpty(starSystem.Planets);

                foreach (var planet in starSystem.Planets)
                {
                    Assert.NotNull(planet.Moons);
                    Assert.NotEmpty(planet.Moons);

                    Assert.NotNull(planet.Name);
                    Assert.NotNull(planet.Atmosphere);
                    
                    Assert.NotEmpty(planet.Name);
                    Assert.NotEmpty(planet.Atmosphere);
                    
                    Assert.NotEqual(double.Epsilon, planet.Mass);
                    Assert.NotEqual(double.Epsilon, planet.Diameter);
                }
            }
        }


    }
}