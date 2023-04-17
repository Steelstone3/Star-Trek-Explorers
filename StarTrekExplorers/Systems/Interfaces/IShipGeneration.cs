using System.Collections.Generic;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface IShipGeneration
    {
        IEnumerable<IShip> GenerateShips(Faction faction);
    }
}