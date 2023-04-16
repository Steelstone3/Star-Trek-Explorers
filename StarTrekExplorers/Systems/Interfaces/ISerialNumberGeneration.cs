using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface ISerialNumberGeneration
    {
        string GenerateSerialNumber(int seed, Faction faction);
    }
}