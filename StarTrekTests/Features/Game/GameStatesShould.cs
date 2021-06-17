using Moq;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game;
using StarTrek.Controllers.Starship;
using StarTrek.Contracts.Game;
using StarTrek.Display;
using StarTrek.States;
using Xunit;
using StarTrek.Contracts.Starships;

namespace StarTrekTests.Features.Game
{
    public class GameStatesShould
    {
        private GameController _gameController = new GameController();
        private Mock<IStarshipController> _starshipControllerMock = new Mock<IStarshipController>();
        private Mock<ILocationController> _locationControllerMock = new Mock<ILocationController>();


        [Fact]
        public void NewGameStatePerformsActions()
        {
            //Given
            var genericOutputHelperMock = new Mock<IGenericOutputHelper>();
            genericOutputHelperMock.Setup(x => x.DisplayMessage("New Game Selected"));

            _gameController.CurrentGameState = new NewGameState(_gameController, _starshipControllerMock.Object, _locationControllerMock.Object, genericOutputHelperMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            genericOutputHelperMock.Verify(x => x.DisplayMessage("New Game Selected"));
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void CharacterCreationStatePerformsActions()
        {
            //Given
            _gameController.CurrentGameState = new CharacterCreationState(_gameController, _starshipControllerMock.Object, _locationControllerMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
        }

        [Fact(Skip = "Functionality needs implementing. Test needs implementing")]
        public void StarshipCreationStatePerformsActions()
        {
            //Given
            //_gameController.CurrentGameState = new StarshipCreationState(_gameController, _starshipController, _locationController);

            //When
            //_gameController.CurrentGameState.StartState();

            //Then
            //Assert.NotNull(_gameController.CurrentGameState);
            //Assert.Equal(something, locationController.Something); etc...
        }

        /*[Fact(Skip = "Functionality needs implementing. Test needs implementing")]
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
        }*/
    }
}




