using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipClasses
    {
        string GetShipClass(int seed, Faction faction);
    }
}