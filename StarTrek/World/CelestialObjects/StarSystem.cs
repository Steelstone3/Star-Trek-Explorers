using System.Collections.Generic;
using StarTrek.Contracts;
using StarTrek.Contracts.World;
using StarTrek.Controllers;

namespace StarTrek.World.CelestialObjects
{
    public class StarSystem : IStarSystem
    {

        public StarSystem(int id, IStarSystemGenerator starSystemGenerator)
        {
            Name = starSystemGenerator.GetName(id);
            Type = starSystemGenerator.GetType(id);
            Mass = starSystemGenerator.GetMass(id);
            Diameter = starSystemGenerator.GetDiameter(id);
        }

        public StarSystem(string name, string type, double mass, double diameter)
        {
            Name = name;
            Type = type;
            Mass = mass;
            Diameter = diameter;
        }

        public string Name { get; private set; }
        public string Type {get;private set;}
        public double Mass { get; private set; }
        public double Diameter { get; private set; }
        public IEnumerable<IPlanet> Planets { get; set; } = new List<IPlanet>();
    }
}