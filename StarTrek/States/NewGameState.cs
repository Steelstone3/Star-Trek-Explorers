using System;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class NewGameState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;

        public NewGameState(IGameController gameController, IStarshipController starshipController, ILocationController locationController) : base(gameController, starshipController, locationController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
        }

        public override void StartState()
        {
            Console.WriteLine("\nNew Game\n");
            StopState();
        }

        public override void StopState()
        {
            Console.WriteLine("\nNew Game State Ended\n");
            GoToState(new CharacterCreationState(_gameController, _starshipController, _locationController));
        }
    }
}