using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public class StarSystemMapState : GameState
    {
        IGameController _gameController;
        IStarshipController _starshipController;
        ILocationController _locationController;

        public StarSystemMapState(IGameController gameController, IStarshipController starshipController, ILocationController locationController) : base(gameController, starshipController, locationController)
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
            //GoToState(new PlanetaryMapState(_gameController,_starshipController, _locationController));
        }
    }
}