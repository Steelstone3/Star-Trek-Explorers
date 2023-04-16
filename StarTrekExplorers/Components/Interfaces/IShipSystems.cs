namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IPhaser Phaser { get; }
        ITorpedo Torpedo { get; }
        IDefense Shield { get; }
        IDefense Hull { get; }
    }
}