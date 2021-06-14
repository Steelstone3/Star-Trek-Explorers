using Moq;
using StarTrek.Controllers.Game;
using StarTrek.Controllers.Starship;
using StarTrek.States;
using Xunit;

namespace StarTrekTests.Features.Game
{
    public class GameStatesShould
    {
        //TODO AH NOT SURE ABOUT THIS TEST SET

        [Fact]
        public void HaveADefaultState()
        {
            //Given
            GameController gameController = new GameController();
            var starshipController = new StarshipController();
            var locationController = new LocationController();
            gameController.CurrentGameState = new NewGameState(gameController, starshipController, locationController);

            //When
            gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(gameController.CurrentGameState);
        }

        [Fact(Skip = "Test not implemented yet")]
        public void StartStopAndGoToNextState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromNewGameStateToCharacterCreationState()
        {
            //Given
            GameController gameController = new GameController();
            var starshipController = new StarshipController();
            var locationController = new LocationController();
            gameController.CurrentGameState = new NewGameState(gameController, starshipController, locationController);

            //When
            gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(gameController.CurrentGameState);
            Assert.Equal(new CharacterCreationState(gameController, starshipController, locationController).GetType().ToString(), gameController.CurrentGameState.GetType().ToString());
        }

        [Fact(Skip = "Currently wizzes on to the next step needs implementing")]
        public void MoveFromCharacterCreationStateToStarshipCreationState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromStarshipCreationStateToNewGameState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromNewGameStateToGalaxyMapState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromGalaxyMapStateToStarSystemMapState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromStarSystemMapStateToGalaxyMapState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromStarSystemMapSystemToPlanetaryMapState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromPlanetaryMapStateToStarSystemMapState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromPlanetaryMapStateToAwayMissionState()
        {
            //Given
            //When
            //Then
        }

        [Fact(Skip = "Test not implemented yet")]
        public void MoveFromAwayMissionStateToPlanetaryMapState()
        {
            //Given
            //When
            //Then
        }
    }
}




