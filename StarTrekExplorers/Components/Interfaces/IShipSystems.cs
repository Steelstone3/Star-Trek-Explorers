namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipSystems
    {
        IPhaser Phaser { get; }
        ITorpedo Torpedo { get; }
        IDamageTaker Shield { get; }
        IDamageTaker Hull { get; }
    }
}