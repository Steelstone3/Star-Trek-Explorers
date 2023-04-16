using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Systems
{
    public interface ISerialNumberGeneration
    {
        string GenerateSerialNumber(int seed, Faction faction);
    }
}