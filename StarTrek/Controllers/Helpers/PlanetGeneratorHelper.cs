using System.Collections.Generic;

namespace StarTrek.Controllers.Helpers
{
    public class PlanetGeneratorHelper
    {
        public PlanetGeneratorHelper()
        {
            Name = new Dictionary<int, string>{
                {0, "Earth"},
                {1, "Mars"},
                {2, "Venus"},
                {3, "Mercury"},
                {4, "Saturn"},
                {5, "Kronos"},
            };

            Atmosphere = new Dictionary<int, string>{
                {0, "0% O2, 0% N, 0% Other"},
                {1, "21% O2, 78% N, 1% Other"},
                {2, "100% O2, 0% N, 0% Other"},
                {3, "50% O2, 45% N, 5% Other"},
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
        public Dictionary<int, string> Atmosphere { get; }
        public Dictionary<int, double> Mass { get; }
        public Dictionary<int, double> Diameter { get; }
    }
}