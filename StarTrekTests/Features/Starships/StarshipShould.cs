using Moq;
using StarTrek.Contracts.Starships.Builders;
using StarTrek.Starships;
using Xunit;

namespace StarTrekTests.Features.Starships
{
    public class StarshipShould
    {
        private Mock<IStarshipBuilder> _starshipBuilder = new Mock<IStarshipBuilder>();

        //Have a start location
        [Fact]
        public void HaveADefaultStartLocation()
        {
            var starship = new Starship();

            Assert.Equal(0, starship.CoordinateLocationX);
            Assert.Equal(0, starship.CoordinateLocationY);
        }

        //Have a start location
        [Theory]
        [InlineData(4, 7)]
        [InlineData(10, 15)]
        [InlineData(2, 70)]
        public void HaveASetStartLocation(int coordinateLocationX, int coordinateLocationY)
        {
            var starship = new Starship(coordinateLocationX, coordinateLocationY);

            Assert.Equal(coordinateLocationX, starship.CoordinateLocationX);
            Assert.Equal(coordinateLocationY, starship.CoordinateLocationY);
        }

        //Is able to move around the galaxy on WASD and arrow keys
        [Theory(Skip ="Move to new location")]
        [InlineData("M")]
        public void WarpToNewLocation(string command)
        {
            
        }

        [Theory(Skip ="Move to new planet")]
        [InlineData("L")]
        public void MoveToNewPlanet(string command)
        {

        }

        //Is able to enter a star system on the enter key
        //Is able to leave a star system on the backspace key
        //Is able to move around a star system with the AD and left/ right arrow keys
        //Is able to perform actions on a planet on the enter key

        //Enters encounters in the galaxy
        //Enters encounters in the star systems

    }
}
