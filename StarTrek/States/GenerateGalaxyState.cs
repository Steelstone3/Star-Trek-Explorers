using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class GenerateGalaxyState : GameState
    {
        IGameController _gameController;
        IStarshipController _starshipController;
        ILocationController _locationController;
        
        public GenerateGalaxyState(IGameController gameController, IStarshipController starshipController, ILocationController locationController) : base(gameController, starshipController, locationController)
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
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