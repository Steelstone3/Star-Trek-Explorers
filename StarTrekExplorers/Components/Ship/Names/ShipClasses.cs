using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Components.Ship.Names
{
    public class ShipClasses : IShipClasses
    {
        private readonly string[] federationShipClasses = new string[] {
            "Galaxy",
            "Intrepid",
            "Defiant",
            "Sovereign",
            "Oberth",
            "Nova",
            "Saber",
            "Miranda",
            "Constellation",
            "Cheyenne",
            "Dakota",
            "Prometheus",
            "Nebula",
            "Luna",
            "Akira",
            "Excelsior",
            "Ambassador",
            "Odyssey",
        };

        private readonly string[] klingonShipClasses = new string[] {
            "Negh'Var",
            "Gel'joQ",
            "B'rel",
            "Felg'ra",
            "K'mpec",
            "K'Vort",
            "Qethla",
            "Torath",
            "Vor'cha",
            "De'nat",
            "DughHegh",
            "Fel'keth",
            "Goralis",
            "Jen'thar",
            "K't'inga",
            "Lotl'eh",
            "Ngapej",
            "Pa'chag",
            "QaDlej",
            "Vodleq",
            "Tormag",
            "Ro'qul",
            "BaH'reth",
            "HajHal",
            "Kel'var",
            "Qa'cheng",
            "Sa'var",
            "To'beq",
            "Yotwl",
            "Bach'chunD",
            "DeSjoh",
            "Po'gach",
            "Sompek",
            "Tro'Qa",
            "Bla'koth",
            "DorHub",
            "Drenok",
            "Qlj'tagh",
            "Vel'taS",
            "Ver'graH"
        };

        public string GetShipClass(int seed, Faction faction)
        {
            RandomGeneration randomGeneration = new();

            switch (faction)
            {
                case Faction.Federation:
                    int federationShipClassIndex = randomGeneration.GetRandomInRange(seed, 0, federationShipClasses.Length - 1);
                    return federationShipClasses[federationShipClassIndex];
                case Faction.KlingonEmpire:
                    int klingonShipClassIndex = randomGeneration.GetRandomInRange(seed, 0, klingonShipClasses.Length - 1);
                    return klingonShipClasses[klingonShipClassIndex];
                default:
                    return "No Ship Class";
            }
        }
    }
}