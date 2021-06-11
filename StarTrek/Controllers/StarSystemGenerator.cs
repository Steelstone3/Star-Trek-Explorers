using StarTrek.Contracts.World;
using StarTrek.Controllers.Helpers;

namespace StarTrek.Controllers
{
    public class StarSystemGenerator : IStarSystemGenerator
    {
        public double GetDiameter(int id)
        {
            var diameter = new StarSystemGeneratorHelper().Diameter;

            if (diameter.ContainsKey(id))
            {
                return diameter[id];
            }
            else
            {
                return diameter[0];
            }
        }

        public double GetMass(int id)
        {
            var masses = new StarSystemGeneratorHelper().Mass;

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
            var name = new StarSystemGeneratorHelper().Name;

            if (name.ContainsKey(id))
            {
                return name[id];
            }
            else
            {
                return name[0];
            }
        }

        public string GetType(int id)
        {
            var type = new StarSystemGeneratorHelper().Type;

            if (type.ContainsKey(id))
            {
                return type[id];
            }
            else
            {
                return type[0];
            }
        }
    }
}