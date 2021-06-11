using System.Collections.Generic;
using Moq;
using StarTrek.Contracts;
using StarTrek.Contracts.World;
using StarTrek.Controllers;
using StarTrek.World;
using StarTrek.World.CelestialObjects;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldShould
    {
        private Mock<IStarSystemGenerator> _starSystemGeneratorMock = new Mock<IStarSystemGenerator>();
        private Mock<IPlanetGenerator> _planetGeneratorMock = new Mock<IPlanetGenerator>();
        private Mock<IMoonGenerator> _moonGeneratorMock = new Mock<IMoonGenerator>();

        //Generate a galaxy (world map)
        //Populate the galaxy with randomly generated persistant star systems
        //Populate the star systems with randomly generated persisitant planets
        //Populate the planets with randomly generated persistant moons
        //Check world generation sequence
        [Fact(Skip ="Not working yet")]
        public void GenerateGalaxyInCorrectOrder()
        {
            //Arrange
            var starSystem = CreateStarSystemFake();
            var mapGeneratorMock = CreateMapGeneratorMock();

            //Act
            var world = new WorldMap(mapGeneratorMock.Object, 
            _starSystemGeneratorMock.Object, 
            _planetGeneratorMock.Object, 
            _moonGeneratorMock.Object);

            //Assert
            mapGeneratorMock.InSequence(new MockSequence());

            mapGeneratorMock.Verify(x => x.GenerateGalaxyMap());
            mapGeneratorMock.Verify(x => x.GenerateGalaxyStarSystems(10, _starSystemGeneratorMock.Object));
            mapGeneratorMock.Verify(x => x.GenerateStarSystemPlanets(starSystem, _planetGeneratorMock.Object));
            mapGeneratorMock.Verify(x => x.GeneratePlanetMoons(starSystem, _moonGeneratorMock.Object));
        }

        private Mock<IMapGenerator> CreateMapGeneratorMock()
        {
            var starSystems = CreateStarSystemFake();

            var mapGeneratorMock = new Mock<IMapGenerator>();
            mapGeneratorMock.Setup(x => x.GenerateGalaxyMap()).Returns(new GalaxyWorldMap());
            mapGeneratorMock.Setup(x => x.GenerateGalaxyStarSystems(10, _starSystemGeneratorMock.Object)).Returns(starSystems);
            mapGeneratorMock.Setup(x => x.GenerateStarSystemPlanets(starSystems, _planetGeneratorMock.Object)).Returns(starSystems);
            mapGeneratorMock.Setup(x => x.GeneratePlanetMoons(starSystems, _moonGeneratorMock.Object)).Returns(starSystems);

            return mapGeneratorMock;
        }

        private IEnumerable<IStarSystem> CreateStarSystemFake()
        {
            var planet = new Planet("Earth", "21% O2, 79% N", 2000, 20000);
            planet.Moons.Add(new Moon("Luna", 230, 2322));

            return new List<IStarSystem>()
            {
                new StarSystem("Bob The Star", "Yellow", 20000, 300000)
                {
                    Planets = new List<IPlanet>()
                    {
                        planet,
                    },
                },
            };
        }
    }
}