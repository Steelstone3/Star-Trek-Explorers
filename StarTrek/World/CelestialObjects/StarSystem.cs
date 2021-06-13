using System.Collections.Generic;
using StarTrek.Contracts.World;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.World.CelestialObjects
{
    public class StarSystem : IStarSystem, ILocation
    {
        public StarSystem(int id, IStarSystemBuilder starSystemGenerator, IEnumerable<IStarSystem> galaxyStarSystems)
        {
            Name = starSystemGenerator.GetName(id);
            Type = starSystemGenerator.GetType(id);
            Mass = starSystemGenerator.GetMass(id);
            Diameter = starSystemGenerator.GetDiameter(id);
            //SetRandomLocation();
            SetUniqueLocation(starSystemGenerator, galaxyStarSystems);
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

        public IEnumerable<IPlanet> Planets { get; set; } = new List<IPlanet>();
        public string Name { get; private set; }
        public string Type {get;private set;}
        public double Mass { get; private set; }
        public double Diameter { get; private set; }
        public int CoordinateLocationX { get; set; }
        public int CoordinateLocationY { get; set; }

        private void SetUniqueLocation(IStarSystemBuilder starSystemGenerator, IEnumerable<IStarSystem> galaxyStarSystems)
        {
            var coordinates = starSystemGenerator.SetUniqueLocation(CoordinateLocationX, CoordinateLocationY, galaxyStarSystems);
            CoordinateLocationX = coordinates.Item1;
            CoordinateLocationY = coordinates.Item2;
        }

        /*private void SetRandomLocation()
        {
            var random = new Random();
            CoordinateLocationX = random.Next(0,100);
            CoordinateLocationY = random.Next(0,100);
        }*/
    }
}