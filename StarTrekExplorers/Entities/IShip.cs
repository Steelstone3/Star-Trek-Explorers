using StarTrekExplorers.Components.Ship.Capabilities;

namespace StarTrekExplorersTests.Entities
{
    public interface IShip
    {
        IIdentification Identification { get; }
        ISystems Systems { get; }
    }
}