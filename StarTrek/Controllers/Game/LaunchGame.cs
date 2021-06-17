using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Controllers.Starship;
using StarTrek.Display;
using StarTrek.States;

namespace StarTrek.Controllers.Game
{
    public class LaunchGame
    {
        public void StartTheGame(IGameController gameController, IStarshipController starshipController, ILocationController locationController)
        {
            gameController.CurrentGameState = new NewGameState(gameController, starshipController, locationController, new GenericOutputHelper(new UserDisplay()));
            gameController.CurrentGameState.StartState();
        }
    }
}

