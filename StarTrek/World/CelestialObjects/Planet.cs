using System.Collections.Generic;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
using StarTrek.Controllers;

namespace StarTrek.World.CelestialObjects
{
    public class Planet : IPlanet
    {
        public Planet(int id, IPlanetBuilder planetGenerator)
        {
            Name = planetGenerator.GetName(id);
            Atmosphere = planetGenerator.GetAtmoshere(id);
            Mass = planetGenerator.GetMass(id);
            Diameter = planetGenerator.GetDiameter(id);
        }

        public Planet(string name, string atmosphere, double mass, double diameter)
        {
            Name = name;
            Atmosphere = atmosphere;
            Mass = mass;
            Diameter = diameter;
        }

        public IEnumerable<IMoon> Moons { get; set; } = new List<IMoon>();
        public string Name { get; private set; }
        public string Atmosphere {get;private set;}
        public double Mass { get; private set; }
        public double Diameter { get; private set; }
    }
}