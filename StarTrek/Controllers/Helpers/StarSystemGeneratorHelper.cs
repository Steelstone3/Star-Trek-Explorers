using System.Collections.Generic;

namespace StarTrek.Controllers.Helpers
{
    public class StarSystemGeneratorHelper
    {
        public StarSystemGeneratorHelper()
        {
            Name = new Dictionary<int, string>{
                {0, "Earth"},
                {1, "Mars"},
                {2, "Venus"},
                {3, "Mercury"},
                {4, "Saturn"},
                {5, "Kronos"},
            };

            Type = new Dictionary<int, string>{
                {0, "Yellow"},
                {1, "Red Dwarf"},
                {2, "Red Giant"},
                {3, "White"},
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
        public Dictionary<int, string> Type { get; }
        public Dictionary<int, double> Mass { get; }
        public Dictionary<int, double> Diameter { get; }
    }
}