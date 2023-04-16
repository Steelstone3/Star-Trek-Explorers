using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public interface IShipClasses
    {
        string GetShipClass(int seed, Faction faction);
    }
}