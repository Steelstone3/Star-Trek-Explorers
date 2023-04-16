using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorers.Presenters
{
    public interface IShipPresenter
    {
        string SelectWeapon(IShipSystems shipSystems);
    }
}