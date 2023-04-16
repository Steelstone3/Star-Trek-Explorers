using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class ShipSystems : IShipSystems
    {
        public IDamageDealer Phaser { get; } = new Phaser();
        public IDamageDealer Torpedo { get; } = new Torpedo();
        public IDamageTaker Shield { get; } = new Shield();
        public IDamageTaker Hull { get; } = new Hull();
    }
}