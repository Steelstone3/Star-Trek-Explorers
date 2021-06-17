using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class CharacterCreationState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;
        private ICrewController _crewController;
       
        public CharacterCreationState(IGameController gameController, IStarshipController starshipController, ILocationController locationController, ICrewController crewController) : base(gameController, starshipController, locationController, crewController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
        }

        public override void StartState()
        {
            _crewController.CreateCrew();
            StopState();
        }

        public override void StopState()
        {
            GoToState(new StarshipCreationState(_gameController, _starshipController, _locationController, _crewController));
        }
    }
}