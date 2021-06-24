using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Services.Character;

namespace StarTrek.States
{
    public class CharacterCreationState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;
        private ICrewController _crewController;
        private IGenericDisplayHelper _genericInputHelper;
       
        public CharacterCreationState(IGameController gameController, 
        IStarshipController starshipController, 
        ILocationController locationController, 
        ICrewController crewController,
        IGenericDisplayHelper genericInputHelper) : base(gameController, starshipController, locationController, crewController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
            _genericInputHelper = genericInputHelper;
        }

        public override void StartState()
        {
            _crewController = new CharacterCreatorService(_crewController, _genericInputHelper).CreatePlayerCrew();

            StopState();
        }

        public override void StopState()
        {
            GoToState(new StarshipCreationState(_gameController, _starshipController, _locationController, _crewController));
        }
    }
}