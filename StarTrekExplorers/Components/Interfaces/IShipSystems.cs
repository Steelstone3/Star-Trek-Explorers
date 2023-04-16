namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IWeapon Phaser { get; }
        IWeapon Torpedo { get; }
        IDefense Shield { get; }
        IDefense Hull { get; }
    }
}