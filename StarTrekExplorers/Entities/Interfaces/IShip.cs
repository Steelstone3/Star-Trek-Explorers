using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorers.Entities.Interfaces
{
    public interface IShip
    {
        IIdentification Identification { get; }
        IShipSystems ShipSystems { get; }
    }
}