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
            var genericDisplayHelper = new GenericDisplayHelper(new UserDisplay());

            gameController.CurrentGameState = new NewGameState(gameController, starshipController, locationController, crewController, genericDisplayHelper);
            gameController.CurrentGameState.StartState();
        }
    }
}

