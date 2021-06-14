using System;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class CharacterCreationState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;

        public CharacterCreationState(IGameController gameController, IStarshipController starshipController, ILocationController locationController) : base(gameController, starshipController, locationController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
        }

        public override void StartState()
        {
            Console.WriteLine("\nCharacter Creation\n");
            StopState();
        }

        public override void StopState()
        {
            GoToState(new StarshipCreationState(_gameController, _starshipController, _locationController));
        }
    }
}