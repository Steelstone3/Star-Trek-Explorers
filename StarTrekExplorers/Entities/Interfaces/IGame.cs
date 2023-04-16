using System.Collections.Generic;

namespace StarTrekExplorers.Entities.Interfaces
{
    public interface IGame
    {
        IShip PlayerShip { get; }
        IEnumerable<IShip> FederationShips { get; }
        IEnumerable<IShip> KlingonShips { get; }
        IUniverse Universe { get; }
    }
}