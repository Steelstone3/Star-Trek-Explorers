using Moq;
using StarTrek.Contracts.Starships;
using StarTrek.Contracts.Starships.Builders;
using StarTrek.Starships;
using Xunit;

namespace StarTrekTests.Features
{
    public class StarshipShould
    {
        private Mock<IStarshipBuilder> _starshipBuilder = new Mock<IStarshipBuilder>();

        //Have a start location
        [Fact(Skip ="Consider how this is being implemented")]
        public void HaveAStartLocation()
        {
            //var mapFactory = new MapFactory();
            //var worldMap = mapFactory.GenerateMap();
            
            var starship = new Starship(_starshipBuilder.Object);
            
            Assert.Equal(0, starship.CoordinateLocationX);
            Assert.Equal(0, starship.CoordinateLocationY);
        }

        //Generate a starship (player character)
        [Fact]
        public void CreateAStarShip()
        {
            //Act
            IStarship starship = new Starship(_starshipBuilder.Object);
            _starshipBuilder.Setup(x => x.BuildStarship()).Returns(starship);

            //Assert
            _starshipBuilder.Verify(x => x.BuildStarship());
        }

        

        //Is able to move around the galaxy on WASD and arrow keys
        //Is able to enter a star system on the enter key
        //Is able to leave a star system on the backspace key
        //Is able to move around a star system with the AD and left/ right arrow keys
        //Is able to perform actions on a planet on the enter key

        //Enters encounters in the galaxy
        //Enters encounters in the star systems

    }
}
