using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorers.Entities.Interfaces
{
    public interface IShip : IDamageDealer, IDamageTaker
    {
        IIdentification Identification { get; }
        IShipSystems ShipSystems { get; }
    }
}