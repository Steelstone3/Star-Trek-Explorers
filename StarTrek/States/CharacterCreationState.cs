using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Controllers.Game.Character.CrewRoles;

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
            string name = string.Empty;

            name = _genericInputHelper.GetStringUserInput("Enter Captain's Name");
            _crewController.AddCrewMember(new Captain(), name);

            name = _genericInputHelper.GetStringUserInput("Enter First Officer's Name");
            _crewController.AddCrewMember(new FirstOfficer(), name);

            name = _genericInputHelper.GetStringUserInput("Enter Head Of Engineering's Name");
            _crewController.AddCrewMember(new HeadOfEngineering(), name);

            name = _genericInputHelper.GetStringUserInput("Enter Head Of Medical's Name");
            _crewController.AddCrewMember(new HeadOfMedical(), name);

            name = _genericInputHelper.GetStringUserInput("Enter Head Of Science's Name");
            _crewController.AddCrewMember(new HeadOfScience(), name);

            name = _genericInputHelper.GetStringUserInput("Enter Head Of Security's Name");
            _crewController.AddCrewMember(new HeadOfSecurity(), name);

            name = _genericInputHelper.GetStringUserInput("Enter Head Of Tactical's Name");
            _crewController.AddCrewMember(new HeadOfTactical(), name);

            StopState();
        }

        public override void StopState()
        {
            GoToState(new StarshipCreationState(_gameController, _starshipController, _locationController, _crewController));
        }
    }
}