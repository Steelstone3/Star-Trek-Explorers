using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World.CelestialObjects
{
    public class Moon : IMoon
    {
        public Moon(int id, IMoonBuilder moonGenerator)
        {
            Name = moonGenerator.GetName(id);
            Diameter = moonGenerator.GetDiameter(id);
            Mass = moonGenerator.GetMass(id);
        }

        public Moon(string name, double mass, double diameter)
        {
            Name = name;
            Mass = mass;
            Diameter = diameter;
        }

        public string Name { get; private set; }
        public double Mass { get; private set; }
        public double Diameter { get; private set; }
    }
}