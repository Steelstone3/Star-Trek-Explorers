using Moq;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Controllers.Game;
using StarTrek.Controllers.Starship;
using StarTrek.States;
using Xunit;

namespace StarTrekTests.Features
{
    public class StateMachineShould
    {
        [Fact]
        public void HaveADefaultState()
        {
            //Arrange
            var starshipController = new Mock<IStarshipController>();
            var crewController = new Mock<ICrewController>();
            var locationController = new Mock<ILocationController>();
            var genericOutputHelper = new Mock<IGenericDisplayHelper>();
            var gameController = new GameController();
            
            //Act
            gameController.CurrentGameState = new NewGameState(gameController, starshipController.Object, locationController.Object, crewController.Object, genericOutputHelper.Object);

            //Assert
            Assert.NotNull(gameController.CurrentGameState);
        }

        [Fact(Skip ="Mock game state and check that start state, stop state and go to next state are executed in sequence")]
        public void PerformStateActions()
        {
            
        }
    }
}