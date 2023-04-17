using System.Collections.Generic;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface IShipGeneration
    {
        IEnumerable<IShip> GenerateShips(IPresenter presenter, Faction faction);
    }
}