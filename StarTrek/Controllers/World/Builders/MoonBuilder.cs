using StarTrek.Contracts.World.Builders;
using StarTrek.Controllers.World.Helpers;

namespace StarTrek.Controllers.World.Builders
{
    public class MoonBuilder : IMoonBuilder
    {
        public double GetDiameter(int id)
        {
            var diameters = new MoonBuilderHelper().Diameter;

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
            var masses = new MoonBuilderHelper().Mass;

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
            var names = new MoonBuilderHelper().Name;

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