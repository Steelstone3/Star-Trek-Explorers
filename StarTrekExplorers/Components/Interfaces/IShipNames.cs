using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IShipNames
    {
        string GetShipName(int seed, Faction faction);
    }
}