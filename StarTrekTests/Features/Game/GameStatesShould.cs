using Moq;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game;
using StarTrek.Contracts.Game;
using StarTrek.States;
using Xunit;
using StarTrek.Contracts.Starships;
using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character;

namespace StarTrekTests.Features.Game
{
    public class GameStatesShould
    {
        private GameController _gameController = new GameController();
        private Mock<IStarshipController> _starshipControllerMock = new Mock<IStarshipController>();
        private Mock<ICrewController> _crewController = new Mock<ICrewController>();
        private Mock<ILocationController> _locationControllerMock = new Mock<ILocationController>();


        [Fact(Skip = "Actually runs the state machine")]
        public void NewGameStatePerformsActions()
        {
            //Given
            var genericDisplayHelperMock = new Mock<IGenericDisplayHelper>();
            genericDisplayHelperMock.Setup(x => x.DisplayMessage("New Game Selected"));

            _gameController.CurrentGameState = new NewGameState(_gameController,
            _starshipControllerMock.Object,
            _locationControllerMock.Object,
            _crewController.Object,
            genericDisplayHelperMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            genericDisplayHelperMock.Verify(x => x.DisplayMessage("New Game Selected"));
        }

        [Fact(Skip = "Actually runs the state machine, So called not invoked AddCrewMember() test is incorrect")]
        public void CharacterCreationStatePerformsActions()
        {
            //Given
            var genericDisplayHelperMock = new Mock<IGenericDisplayHelper>();
            //genericDisplayHelperMock.Setup(x => x.GetStringUserInput("Enter Captain's Name")).Returns("Bob");
            genericDisplayHelperMock.Setup(x => x.GetStringUserInput("Enter First Officer's Name")).Returns("Dave");

            var crewRoleMock = new Mock<ICrewRole>();
            crewRoleMock.Setup(x => x.Rank).Returns("First Officer");
            crewRoleMock.Setup(x => x.Role).Returns("Commander");

            var crewControllerMock = new Mock<ICrewController>();
            crewControllerMock.Setup(x => x.AddCrewMember(crewRoleMock.Object, "Dave"));

            _gameController.CurrentGameState = new CharacterCreationState(_gameController,
            _starshipControllerMock.Object,
            _locationControllerMock.Object,
            crewControllerMock.Object,
            genericDisplayHelperMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            //genericDisplayHelperMock.Verify(x => x.GetStringUserInput("Enter Captain's Name"));
            genericDisplayHelperMock.Verify(x => x.GetStringUserInput("Enter First Officer's Name"));
            //crewControllerMock.Verify(x => x.AddCrewMember(crewRoleMock.Object, "Bob"));
            crewControllerMock.Verify(x => x.AddCrewMember(crewRoleMock.Object, "Dave"));
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




