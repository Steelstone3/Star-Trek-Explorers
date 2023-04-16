using System.Collections.Generic;
using System.Formats.Asn1;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Game : IGame
    {
        public Game(IPresenter presenter)
        {
            PlayerShip = new Ship(presenter, new RandomGeneration().GetSeed(), Faction.Federation);
        }

        public IShip PlayerShip { get; }
        public IEnumerable<IShip> FederationShips { get; } = new List<IShip>();
        public IEnumerable<IShip> KlingonShips { get; } = new List<IShip>();
        public IUniverse Universe { get; } = new Universe();
    }
}