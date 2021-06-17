using StarTrek.Controllers.Game;
using StarTrek.Controllers.Starship;
using StarTrek.States;
using Xunit;

namespace StarTrekTests.Features.Game
{
    public class GameStatesShould
    {
        private GameController _gameController = new GameController();
        private StarshipController _starshipController = new StarshipController();
        private LocationController _locationController = new LocationController();

        [Fact(Skip = "Test the output of the state")]
        public void NewGameStatePerformsActions()
        {
            //Given
            //_gameController.CurrentGameState = new NewGameState(_gameController, _starshipController, _locationController, MockedControllersToRunVerificationTests);
            _gameController.CurrentGameState = new NewGameState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void CharacterCreationStatePerformsActions()
        {
            //Given
            _gameController.CurrentGameState = new CharacterCreationState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void StarshipCreationStatePerformsActions()
        {
              //Given
            _gameController.CurrentGameState = new StarshipCreationState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void GenerateGalaxyStatePerformsActions()
        {
             //Given
            _gameController.CurrentGameState = new GenerateGalaxyState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void GalaxyMapStatePerformsActions()
        {
            //Given
            _gameController.CurrentGameState = new GalaxyMapState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void StarsystemMapStatePerformsActions()
        {
            //Given
            _gameController.CurrentGameState = new StarSystemMapState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void PlanetaryMapStateStatePerformsActions()
        {
             //Given
            _gameController.CurrentGameState = new PlanetaryMapState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void AwayMissionStatePerformsActions()
        {
            //Given
            _gameController.CurrentGameState = new AwayMissionState(_gameController, _starshipController, _locationController);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }
    }
}




