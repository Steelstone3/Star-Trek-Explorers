using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class NewGameState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;
        private ICrewController _crewController;
        private IGenericOutputHelper _genericOutputHelper;

        public NewGameState(IGameController gameController, IStarshipController starshipController, ILocationController locationController, ICrewController crewController, IGenericOutputHelper genericInputHelper) : base(gameController, starshipController, locationController, crewController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
            _genericOutputHelper = genericInputHelper;
        }

        public override void StartState()
        {
           _genericOutputHelper.DisplayMessage("New Game Selected");

            StopState();
        }

        public override void StopState()
        {
            Console.WriteLine("\nNew Game State Ended\n");
            GoToState(new CharacterCreationState(_gameController, _starshipController, _locationController, _crewController));
        }
    }
}