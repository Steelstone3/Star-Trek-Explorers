using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Identification : IIdentification
    {
        public Identification(int seed, Faction faction)
        {
            Faction = faction;
            SerialNumber = new SerialNumberGeneration().GenerateSerialNumber(seed, faction);
            Name = "";
            Class = "";
        }

        public Faction Faction { get; }
        public string SerialNumber { get; }
        public string Name { get; }
        public string Class { get; }
    }
}