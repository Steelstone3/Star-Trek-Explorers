using System.Collections.Generic;
using Moq;
using StarTrek.Contracts;
using StarTrek.World;
using Xunit;

namespace StarTrekTests.Features
{
    public class WorldShould
    {
        //Generate a galaxy (world map)
        [Fact]
        public void GenerateAGalaxy()
        {
            //Arrange
            var mapGeneratorMock = CreateMapGeneratorMock();

            //Act
            var world = new WorldMap(mapGeneratorMock.Object);

            //Assert
            mapGeneratorMock.Verify(x => x.GenerateGalaxyMap());
        }

        //Populate the galaxy with randomly generated persistant star systems
        [Fact]
        public void PopulateGalaxyWithStarSystems()
        {
            //Arrange
            var mapGeneratorMock = CreateMapGeneratorMock();

            //Act
            var world = new WorldMap(mapGeneratorMock.Object);

            //Assert
            mapGeneratorMock.Verify(x => x.GeneratePopulatedGalaxyStarSystems());
        }

        //Check world generation sequence
        [Fact]
        public void GenerateInCorrectOrder()
        {
            //Arrange
            var mapGeneratorMock = CreateMapGeneratorMock();

            //Act
            var world = new WorldMap(mapGeneratorMock.Object);

            //Assert
            mapGeneratorMock.InSequence(new MockSequence());

            mapGeneratorMock.Verify(x => x.GenerateGalaxyMap());
            mapGeneratorMock.Verify(x => x.GeneratePopulatedGalaxyStarSystems());
        }

        private Mock<IMapGenerator> CreateMapGeneratorMock()
        {
            var mapGeneratorMock = new Mock<IMapGenerator>();
            mapGeneratorMock.Setup(x => x.GenerateGalaxyMap()).Returns(new GalaxyWorldMap());
            mapGeneratorMock.Setup(x => x.GeneratePopulatedGalaxyStarSystems()).Returns(new List<IStarSystems>());

            return mapGeneratorMock;
        }
    }
}