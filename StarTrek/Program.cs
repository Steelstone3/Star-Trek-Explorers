using StarTrek.Controllers.Game;
using StarTrek.Controllers.Starship;
using StarTrek.States;

namespace StarTrek
{
    class Program
    {
        static void Main(string[] args)
        {
            new LaunchGame().StartTheGame(new GameController(), new StarshipController(), new LocationController());
        }
    }
}