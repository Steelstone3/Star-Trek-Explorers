using StarTrekExplorers.Components.Ship.Capabilities;
using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        public Ship(int seed, Faction faction)
        {
            Identification = new Identification(seed, faction);
        }

        public IIdentification Identification { get; }
        public ISystems Systems { get; } = new Systems();
    }
}