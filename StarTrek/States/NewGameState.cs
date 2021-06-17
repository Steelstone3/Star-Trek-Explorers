using System;
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
        private IGenericOutputHelper _genericOutputHelper;

        public NewGameState(IGameController gameController, IStarshipController starshipController, ILocationController locationController, IGenericOutputHelper genericInputHelper) : base(gameController, starshipController, locationController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
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
            GoToState(new CharacterCreationState(_gameController, _starshipController, _locationController));
        }
    }
}