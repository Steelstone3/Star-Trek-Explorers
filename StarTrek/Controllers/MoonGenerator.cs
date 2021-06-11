using StarTrek.Contracts.World;
using StarTrek.Controllers.Helpers;

namespace StarTrek.Controllers
{
    public class MoonGenerator : IMoonGenerator
    {
        public double GetDiameter(int id)
        {
            var diameters = new MoonGeneratorHelper().Diameter;

            if (diameters.ContainsKey(id))
            {
                return diameters[id];
            }
            else
            {
                return diameters[0];
            }
        }

        public double GetMass(int id)
        {
            var masses = new MoonGeneratorHelper().Mass;

            if (masses.ContainsKey(id))
            {
                return masses[id];
            }
            else
            {
                return masses[0];
            }
        }

        public string GetName(int id)
        {
            var names = new MoonGeneratorHelper().Name;

            if (names.ContainsKey(id))
            {
                return names[id];
            }
            else
            {
                return names[0];
            }
        }
    }
}