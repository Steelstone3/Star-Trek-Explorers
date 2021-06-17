using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;

namespace StarTrek.States
{
    public abstract class GameState : IGameState
    {
        private readonly IGameController _gameController;
        private readonly ILocationController _locationController;
        private readonly IStarshipController _starshipController;
        private readonly ICrewController _crewController;
        

        public GameState(IGameController gameController, IStarshipController starshipController, ILocationController locationController, ICrewController crewController )
        {
            _gameController = gameController;
            _starshipController = starshipController;
            _locationController = locationController;
            _crewController = crewController;
        }

        public void GoToState(IGameState theState)
        {
            _gameController.CurrentGameState = theState;
            _gameController.CurrentGameState.StartState();
        }

        public abstract void StartState();

        public abstract void StopState();
    }
}