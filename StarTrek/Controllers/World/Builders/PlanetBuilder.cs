using StarTrek.Contracts.World.Builders;
using StarTrek.Controllers.World.Helpers;

namespace StarTrek.Controllers.World.Builders
{
    public class PlanetBuilder : IPlanetBuilder
    {
        public string GetAtmoshere(int id)
        {
            var atmospheres = new PlanetBuilderHelper().Atmosphere;

            if (atmospheres.ContainsKey(id))
            {
                return atmospheres[id];
            }
            else
            {
                return atmospheres[0];
            }
        }

        public double GetDiameter(int id)
        {
            var diameters = new PlanetBuilderHelper().Diameter;

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
            var masses = new PlanetBuilderHelper().Mass;

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
            var names = new PlanetBuilderHelper().Name;

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