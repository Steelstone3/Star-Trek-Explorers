using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorers.Presenters.Interfaces
{
    public interface IShipPresenter
    {
        string SelectWeapon(IShipSystems shipSystems);
    }
}