using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class GalaxyMapState : GameState
    {
        IGameController _gameController;
        IStarshipController _starshipController;
        ILocationController _locationController;
        ICrewController _crewController;

        public GalaxyMapState(IGameController gameController, 
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
            //GoToState(new StarSystemMapState(_gameController, _starshipController, _locationController));
        }
    }
}