using StarTrek.Contracts.Character;
using StarTrek.Contracts.Game;
using StarTrek.Contracts.Starships;
using StarTrek.Display;
using StarTrek.States;

namespace StarTrek.Controllers.Game
{
    public class LaunchGame
    {
        public void StartTheGame(IGameController gameController, IStarshipController starshipController, ILocationController locationController, ICrewController crewController)
        {
            gameController.CurrentGameState = new NewGameState(gameController, starshipController, locationController, crewController, new GenericOutputHelper(new UserDisplay()));
            gameController.CurrentGameState.StartState();
        }
    }
}

