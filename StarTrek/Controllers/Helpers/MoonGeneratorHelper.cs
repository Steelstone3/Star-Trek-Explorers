using System.Collections.Generic;

namespace StarTrek.Controllers.Helpers
{
    public class MoonGeneratorHelper
    {
        public MoonGeneratorHelper()
        {
            Name = new Dictionary<int, string>{
                {0, "Luna"},
                {1, "Titan"},
            };

            Mass = new Dictionary<int, double>{
                {0, 26545.23},
                {1, 1056.45},
                {2, 32768.89},
                {3, 93294.78},
            };

            Diameter = new Dictionary<int, double>{
                {0, 56782.23},
                {1, 98562.45},
                {2, 5667.89},
                {3, 34579.78},
            };
        }

        public Dictionary<int, string> Name { get; }
        public Dictionary<int, double> Mass { get; }
        public Dictionary<int, double> Diameter { get; }
    }
}