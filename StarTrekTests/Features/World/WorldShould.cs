using System.Collections.Generic;
using Moq;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.World;
using Xunit;

namespace StarTrekTests.Features.World
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
            _starSystemMock.Object,
            _planetBuilderMock.Object,
            _moonBuilderMock.Object);

            //Assert
            mapGeneratorMock.InSequence(new MockSequence());

            mapGeneratorMock.Verify(x => x.BuildInitialGalaxyMap());
            mapGeneratorMock.Verify(x => x.BuildGalaxyStarSystems(250, _starSystemBuilderMock.Object, _starSystemMock.Object));
            mapGeneratorMock.Verify(x => x.BuildStarSystemPlanets(_starSystemMock.Object, _planetBuilderMock.Object));
            mapGeneratorMock.Verify(x => x.BuildPlanetMoons(_starSystemMock.Object, _moonBuilderMock.Object));
        }

        private Mock<IMapFactory> CreateMapGeneratorMock()
        {
            var mapGeneratorMock = new Mock<IMapFactory>();
            mapGeneratorMock.Setup(x => x.BuildInitialGalaxyMap()).Returns(new GalaxyWorldMap());
            mapGeneratorMock.Setup(x => x.BuildGalaxyStarSystems(250, _starSystemBuilderMock.Object, _starSystemMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.BuildStarSystemPlanets(_starSystemMock.Object, _planetBuilderMock.Object)).Returns(_starSystemMock.Object);
            mapGeneratorMock.Setup(x => x.BuildPlanetMoons(_starSystemMock.Object, _moonBuilderMock.Object)).Returns(_starSystemMock.Object);
            return mapGeneratorMock;
        }
    }
}