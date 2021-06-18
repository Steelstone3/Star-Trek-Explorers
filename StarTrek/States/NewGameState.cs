using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Display;

namespace StarTrek.States
{
    public class NewGameState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;
        private ICrewController _crewController;
        private IGenericDisplayHelper _genericDisplayHelper;

        public NewGameState(IGameController gameController, 
        IStarshipController starshipController, 
        ILocationController locationController, 
        ICrewController crewController, 
        IGenericDisplayHelper genericDisplayHelper) : base(gameController, starshipController, locationController, crewController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
            _genericDisplayHelper = genericDisplayHelper;
        }

        public override void StartState()
        {
            _genericDisplayHelper.DisplayMessage("New Game Selected");

            StopState();
        }

        public override void StopState()
        {
            GoToState(new CharacterCreationState(_gameController, _starshipController, _locationController, _crewController, new GenericDisplayHelper(new UserDisplay())));
        }
    }
}