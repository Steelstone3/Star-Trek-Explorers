namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IWeapon Phaser { get; }
        IWeapon Torpedo { get; }
        IDamageTaker Shield { get; }
        IDamageTaker Hull { get; }
    }
}