using StarTrek.Contracts.Game;

namespace StarTrek.Controllers.Game
{
    public class GameController : IGameController
    {
        public IGameState CurrentGameState
        {
            get;
            set;
        }
    }
}