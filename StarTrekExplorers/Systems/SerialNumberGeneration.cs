using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Systems.Interfaces;

namespace StarTrekExplorersTests.Systems
{
    public class SerialNumberGeneration : ISerialNumberGeneration
    {
        public string GenerateSerialNumber(int seed, Faction faction)
        {
            RandomGeneration randomGeneration = new();
            string serialNumber = randomGeneration.GetRandomInRange(seed, 10000, 100000).ToString();

            return faction switch
            {
                Faction.Federation => "USS-" + serialNumber,
                Faction.KlingonEmpire => "IKS-" + serialNumber,
                _ => "No Faction",
            };
        }
    }
}