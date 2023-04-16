using System;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Components.Ship.Names
{
    public class ShipNames : IShipNames
    {
        private readonly string[] federationShipNames = new string[] {
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

        private readonly string[] klingonShipNames = new string[] 
        {
            "Amar",
            "B'Moth",
            "B'rel",
            "Buruk",
            "Ch'Tang",
            "Groth",
            "Heghta",
            "K'Vada",
            "K'Vort",
            "K't'inga",
            "Ki'tang",
            "Koraga",
            "M'Char",
            "NeghVar",
            "Ning'tao",
            "Orantho",
            "Pagh",
            "Rotarran",
            "Slivin",
            "Somraw",
            "T'Acog",
            "Tagana",
            "Toh'Kaht",
            "Voodieh",
            "Vor'cha",
            "Vorn"
        };

        public string GetShipName(int seed, Faction faction)
        {
            RandomGeneration randomGeneration = new();

            switch (faction)
            {
                case Faction.Federation:
                    int federationShipNamesIndex = randomGeneration.GetRandomInRange(seed, 0, federationShipNames.Length - 1);
                    return federationShipNames[federationShipNamesIndex];
                case Faction.KlingonEmpire:
                    int klingonShipNamesIndex = randomGeneration.GetRandomInRange(seed, 0, klingonShipNames.Length - 1);
                    return klingonShipNames[klingonShipNamesIndex];
                default:
                    return "No Ship Name";
            }
        }
    }
}