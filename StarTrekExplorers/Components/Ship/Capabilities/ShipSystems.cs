using System.Net;
using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class ShipSystems : IShipSystems
    {
        public IWeapon Phaser { get; } = new Phaser();
        public IWeapon Torpedo { get; } = new Torpedo();
        public IDamageTaker Shield { get; } = new Shield();
        public IDamageTaker Hull { get; } = new Hull();
    }
}