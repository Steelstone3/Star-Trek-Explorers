using StarTrekExplorers.Components.Ship.Names;

namespace StarTrekExplorersTests.Entities
{
    public class Identification : IIdentification
    {
        public Identification(Faction faction)
        {
            Faction = faction;
            SerialNumber = "";
            Name = "";
            Class = "";
        }

        public Faction Faction { get; }
        public string SerialNumber { get; }
        public string Name { get; }
        public string Class { get; }
    }
}