using StarTrekExplorers.Components.Ship.Capabilities;
using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        public Ship(Faction faction)
        {
            Identification = new Identification(faction);
        }

        public IIdentification Identification { get; }
        public ISystems Systems { get; } = new Systems();
    }
}