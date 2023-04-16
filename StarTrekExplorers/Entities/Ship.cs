using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        public Ship(int seed, Faction faction)
        {
            Identification = new Identification(seed, faction);
        }

        public IIdentification Identification { get; }
        public IShipSystems ShipSystems { get; } = new ShipSystems();
    }
}