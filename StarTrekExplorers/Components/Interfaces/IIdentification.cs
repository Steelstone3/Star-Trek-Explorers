using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IIdentification
    {
        Faction Faction { get; }
        string Name { get; }
        string ShipClass { get; }
        string SerialNumber { get; }
    }
}