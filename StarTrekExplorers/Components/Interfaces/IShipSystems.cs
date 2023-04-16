namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IDamageDealer Phaser { get; }
        IDamageDealer Torpedo { get; }
        IDamageTaker Shield { get; }
        IDamageTaker Hull { get; }
    }
}