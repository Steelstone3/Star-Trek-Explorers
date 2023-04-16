using System;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.Ship.Names
{
    public class ShipNames : IShipNames
    {
        private readonly string[] federationNames = new string[] {
            "Akira",
            "Archon",
            "Atlantis",
            "Avenger",
            "Aventine",
            "Bozeman",
            "Centaur",
            "Challenger",
            "Chekov",
            "Chimera",
            "Columbia",
            "Cortez",
            "Curry",
            "Dauntless",
            "Defiant",
            "Discovery",
            "Endeavour",
            "Enterprise",
            "Excalibur",
            "Excelsio",
            "Excelsior",
            "Farragut",
            "Galaxy",
            "Grissom",
            "Hood",
            "Horizon",
            "Intrepid",
            "Lakota",
            "Lexington",
            "Majestic",
            "Melbourne",
            "Merrimack",
            "Monitor",
            "Nebula",
            "Newton",
            "Nova",
            "Odyssey",
            "Pasteur",
            "Potemkin",
            "Prometheus",
            "Proxima",
            "Relativity",
            "Saratoga",
            "Stargazer",
            "Sutherland",
            "Thunderchild",
            "Titan",
            "Valiant",
            "Voyager",
            "Yorktown"
        };

        public string GetShipName(int seed, Faction faction)
        {
            RandomGeneration randomGeneration = new();
            int index = randomGeneration.GetRandomInRange(seed, 0, federationNames.Length - 1);

            return federationNames[index];
        }
    }
}