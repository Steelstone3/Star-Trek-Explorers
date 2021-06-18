using StarTrek.Controllers.Game;
using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.Factories;
using StarTrek.Controllers.Starship;
using StarTrek.Display;

namespace StarTrek
{
    class Program
    {
        static void Main(string[] args)
        {
            new LaunchGame().StartTheGame(new GameController(), 
            new StarshipController(), 
            new LocationController(), 
            new CrewController(new CharacterFactory()));
        }
    }
}