using Moq;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game;
using StarTrek.Contracts.Game;
using StarTrek.States;
using Xunit;
using StarTrek.Contracts.Starships;
using StarTrek.Contracts.Character;

namespace StarTrekTests.Features.Game
{
    public class GameStatesShould
    {
        private GameController _gameController = new GameController();
        private Mock<IStarshipController> _starshipControllerMock = new Mock<IStarshipController>();
        private Mock<ICrewController> _crewController = new Mock<ICrewController>();
        private Mock<ILocationController> _locationControllerMock = new Mock<ILocationController>();


        [Fact]
        public void NewGameStatePerformsActions()
        {
            //Given
            var genericOutputHelperMock = new Mock<IGenericOutputHelper>();
            genericOutputHelperMock.Setup(x => x.DisplayMessage("New Game Selected"));

            _gameController.CurrentGameState = new NewGameState(_gameController, _starshipControllerMock.Object, _locationControllerMock.Object,_crewController.Object,genericOutputHelperMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            genericOutputHelperMock.Verify(x => x.DisplayMessage("New Game Selected"));
        }

        [Fact(Skip ="Need to flesh this out not sure what I want yet")]
        public void CharacterCreationStatePerformsActions()
        {
            //Given
            /*var crewRoleMock = new Mock<ICrewRole>();
            crewRoleMock.SetupAllProperties();

            var crewMemberMock = new Mock<ICrewMember>();
            crewMemberMock.Name = "Jeff";

            var crewComplimentMock = new Mock<ICrewCompliment>();
            crewComplimentMock.Setup(x => x.Captain).Returns(crewMemberMock.Object);
            crewComplimentMock.Setup(x => x.FirstOfficer).Returns(crewMemberMock.Object);
            crewComplimentMock.Setup(x => x.HeadOfEngineering).Returns(crewMemberMock.Object);

            var characterFactoryMock = new Mock<ICharacterFactory>();
            characterFactoryMock.Setup(x => x.CreateCrewMember(crewRoleMock.Object, "Jeff")).Returns(crewMemberMock.Object);
            characterFactoryMock.Setup(x => x.AddCrewMemberToCrewCompliment(crewComplimentMock.Object, crewMemberMock.Object)).Returns(crewComplimentMock.Object);

            var starshipController = new StarshipController();

            _gameController.CurrentGameState = new CharacterCreationState(_gameController, starshipController, _locationControllerMock.Object, characterFactoryMock.Object);

            //When
            _gameController.CurrentGameState.StartState();

            //Then
            Assert.NotNull(_gameController.CurrentGameState);
            characterFactoryMock.Verify(x => x.CreateCrewMember(crewRoleMock.Object, "Jeff"));
            characterFactoryMock.Verify(x => x.AddCrewMemberToCrewCompliment(crewComplimentMock.Object, crewMemberMock.Object));*/
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




