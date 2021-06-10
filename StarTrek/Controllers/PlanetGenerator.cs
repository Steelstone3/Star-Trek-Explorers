using StarTrek.Contracts;
using StarTrek.World.CelestialObjects;

namespace StarTrek.Controllers
{
    public class PlanetGenerator : IPlanetGenerator
    {
        public string GetAtmoshere(int id)
        {
            var atmospheres = new PlanetGeneratorHelper().Atmosphere;

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
            var diameters = new PlanetGeneratorHelper().Diameter;

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
            var masses = new PlanetGeneratorHelper().Mass;

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
            var names = new PlanetGeneratorHelper().Name;

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