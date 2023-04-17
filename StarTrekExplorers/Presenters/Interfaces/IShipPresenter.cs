using System.Collections.Generic;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Entities.Interfaces;

namespace StarTrekExplorers.Presenters.Interfaces
{
    public interface IShipPresenter
    {
        string SelectWeapon(IShipSystems shipSystems);
        void PrintShipName(IShip ship);
        void PrintShipNames(IEnumerable<IShip> ships);
    }
}