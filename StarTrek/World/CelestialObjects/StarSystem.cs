using System.Collections.Generic;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World.CelestialObjects
{
    public class StarSystem : IStarSystem, ILocation
    {

        public StarSystem(int id, IStarSystemBuilder starSystemGenerator)
        {
            Name = starSystemGenerator.GetName(id);
            Type = starSystemGenerator.GetType(id);
            Mass = starSystemGenerator.GetMass(id);
            Diameter = starSystemGenerator.GetDiameter(id);
        }

        public StarSystem(string name, string type, double mass, double diameter, int coordinateLocationX, int coordinateLocationY)
        {
            Name = name;
            Type = type;
            Mass = mass;
            Diameter = diameter;
            CoordinateLocationX = coordinateLocationX;
            CoordinateLocationY = coordinateLocationY;
        }

        public string Name { get; private set; }
        public string Type {get;private set;}
        public double Mass { get; private set; }
        public double Diameter { get; private set; }
        public IEnumerable<IPlanet> Planets { get; set; } = new List<IPlanet>();
        public int CoordinateLocationX { get; set; }
        public int CoordinateLocationY { get; set; }
    }
}