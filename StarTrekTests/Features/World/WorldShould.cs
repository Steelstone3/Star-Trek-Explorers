using System.Collections.Generic;
using Moq;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.World;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldShould
    {
        private Mock<IStarSystemBuilder> _starSystemBuilderMock = new Mock<IStarSystemBuilder>();
        private Mock<IPlanetBuilder> _planetBuilderMock = new Mock<IPlanetBuilder>();
        private Mock<IMoonBuilder> _moonBuilderMock = new Mock<IMoonBuilder>();
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
            _starSystemBuilderMock.Object, 
            _planetBuilderMock.Object, 
            _moonBuilderMock.Object);

            //Assert
            mapGeneratorMock.InSequence(new MockSequence());

            mapGeneratorMock.Verify(x => x.GenerateGalaxyMap());
            mapGeneratorMock.Verify(x => x.GenerateGalaxyStarSystems(10, _starSystemBuilderMock.Object));
            mapGeneratorMock.Verify(x => x.GenerateStarSystemPlanets(_starSystemMock.Object, _planetBuilderMock.Object));
            mapGeneratorMock.Verify(x => x.GeneratePlanetMoons(_starSystemMock.Object, _moonBuilderMock.Object));
            mapGeneratorMock.Verify(x => x.DistributeStarSystems());
        }

        private Mock<IMapFactory> CreateMapGeneratorMock()
        {
            var mapGeneratorMock = new Mock<IMapFactory>();
            mapGeneratorMock.Setup(x => x.GenerateGalaxyMap()).Returns(new GalaxyWorldMap());
            mapGeneratorMock.Setup(x => x.GenerateGalaxyStarSystems(10, _starSystemBuilderMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.GenerateStarSystemPlanets(_starSystemMock.Object, _planetBuilderMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.GeneratePlanetMoons(_starSystemMock.Object, _moonBuilderMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.DistributeStarSystems());
            return mapGeneratorMock;
        }
    }
}