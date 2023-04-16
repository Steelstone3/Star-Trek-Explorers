using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Entities
{
    public interface IIdentification
    {
        Faction Faction { get; }
        string Name { get; }
        string Class { get; }
        string SerialNumber { get; }
    }
}