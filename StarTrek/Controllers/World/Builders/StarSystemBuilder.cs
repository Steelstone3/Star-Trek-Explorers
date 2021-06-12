using StarTrek.Contracts.World.Builders;
using StarTrek.Controllers.World.Helpers;

namespace StarTrek.Controllers.World.Builders
{
    public class StarSystemBuilder : IStarSystemBuilder
    {
        public double GetDiameter(int id)
        {
            var diameter = new StarSystemBuilderHelper().Diameter;

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
            var masses = new StarSystemBuilderHelper().Mass;

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
            var name = new StarSystemBuilderHelper().Name;

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
            var type = new StarSystemBuilderHelper().Type;

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