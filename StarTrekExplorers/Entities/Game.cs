using System.Collections.Generic;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorers.Systems;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Game : IGame
    {
        public Game(IPresenter presenter)
        {
            ShipGeneration shipGeneration = new();
            PlayerShip = new PlayerShip(presenter, new RandomGeneration().GetSeed(), Faction.Federation);
            FederationShips = shipGeneration.GenerateShips(presenter, Faction.Federation);
            KlingonShips = shipGeneration.GenerateShips(presenter, Faction.KlingonEmpire);
        }

        public IShip PlayerShip { get; }
        public IEnumerable<IShip> FederationShips { get; }
        public IEnumerable<IShip> KlingonShips { get; }
        public IUniverse Universe { get; } = new Universe();
    }
}