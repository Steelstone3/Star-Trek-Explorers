namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IPhaser Phaser { get; }
        ITorpedo Torpedo { get; }
        IShield Shield { get; }
        IHull Hull { get; }
    }
}