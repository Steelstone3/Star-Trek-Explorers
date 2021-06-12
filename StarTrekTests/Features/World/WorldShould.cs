using System.Collections.Generic;
using Moq;
using StarTrek.Contracts;
using StarTrek.Contracts.World;
using StarTrek.World;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldShould
    {
        private Mock<IStarSystemGenerator> _starSystemGeneratorMock = new Mock<IStarSystemGenerator>();
        private Mock<IPlanetGenerator> _planetGeneratorMock = new Mock<IPlanetGenerator>();
        private Mock<IMoonGenerator> _moonGeneratorMock = new Mock<IMoonGenerator>();
        private Mock<IEnumerable<IStarSystem>> _starSystemMock = new Mock<IEnumerable<IStarSystem>>();

        //Generate a galaxy (world map)
        //Populate the galaxy with randomly generated persistant star systems
        //Populate the star systems with randomly generated persisitant planets
        //Populate the planets with randomly generated persistant moons
        //Distibute the star systems across the galaxy (world map)
        //Check world generation sequence
        [Fact]
        public void GenerateGalaxyInCorrectOrder()
        {
            //Arrange
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
            mapGeneratorMock.Verify(x => x.GenerateStarSystemPlanets(_starSystemMock.Object, _planetGeneratorMock.Object));
            mapGeneratorMock.Verify(x => x.GeneratePlanetMoons(_starSystemMock.Object, _moonGeneratorMock.Object));
            mapGeneratorMock.Verify(x => x.DistributeStarSystems());
        }

        private Mock<IMapGenerator> CreateMapGeneratorMock()
        {
            var mapGeneratorMock = new Mock<IMapGenerator>();
            mapGeneratorMock.Setup(x => x.GenerateGalaxyMap()).Returns(new GalaxyWorldMap());
            mapGeneratorMock.Setup(x => x.GenerateGalaxyStarSystems(10, _starSystemGeneratorMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.GenerateStarSystemPlanets(_starSystemMock.Object, _planetGeneratorMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.GeneratePlanetMoons(_starSystemMock.Object, _moonGeneratorMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.DistributeStarSystems());
            return mapGeneratorMock;
        }
    }
}