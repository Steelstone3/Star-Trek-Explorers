using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class GenerateGalaxyState : GameState
    {
        IGameController _gameController;
        IStarshipController _starshipController;
        ILocationController _locationController;
        ICrewController _crewController;
        
        public GenerateGalaxyState(IGameController gameController, 
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
            StopState();
        }

        public override void StopState()
        {
            //GoToState(new GalaxyMapState(_gameController, _starshipController, _locationController));
        }
    }
}