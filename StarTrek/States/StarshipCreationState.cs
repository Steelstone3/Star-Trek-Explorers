using System;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class StarshipCreationState : GameState
    {
        private IGameController _gameController;
        private IStarshipController _starshipController;
        private ILocationController _locationController;
        private ICrewController _crewController;

        public StarshipCreationState(IGameController gameController, 
        IStarshipController starshipController, 
        ILocationController locationController,
        ICrewController crewController) : base(gameController, starshipController, locationController, crewController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
        }

        public override void StartState()
        {
            Console.WriteLine("\nCharacter Creation\n");
            StopState();
        }

        public override void StopState()
        {
            //GoToState(new GenerateGalaxyState(_gameController,_starshipController,_locationController));
        }
    }
}