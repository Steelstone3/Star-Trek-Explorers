using StarTrekExplorers.Components.Ship.Capabilities;

namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        public IIdentification Identification { get; } = new Identification();
        public ISystems Systems { get; } = new Systems();
    }
}