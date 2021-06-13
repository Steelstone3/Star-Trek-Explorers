using System;
using System.Collections.Generic;
using System.Linq;
using StarTrek.Contracts.World.Builders;
using StarTrek.Contracts.World.CelestialBodies;
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

        public Tuple<int, int> SetUniqueLocation(int coordinateLocationX, int coordinateLocationY, IEnumerable<IStarSystem> starSystems)
        {
            var coordinates = GetAllStarSystemCoordinates(starSystems);
            return GetUniqueCoordinates(coordinateLocationX, coordinateLocationY, coordinates);
        }

        private List<Tuple<int, int>> GetAllStarSystemCoordinates(IEnumerable<IStarSystem> starSystems)
        {
            var coordinates = new List<Tuple<int, int>>();

            foreach (var starSystem in starSystems)
            {
                coordinates.Add(new Tuple<int, int>(starSystem.CoordinateLocationX, starSystem.CoordinateLocationY));
            }
            
            coordinates.Sort();

            return coordinates;
        }

        private Tuple<int,int> GetUniqueCoordinates(int coordinateLocationX, int coordinateLocationY, List<Tuple<int, int>> coordinates)
        {
            foreach(var coordinate in coordinates)
            {
                if(coordinate.Item1 == coordinateLocationX && coordinate.Item2 == coordinateLocationY)
                {
                    if(coordinate.Item1 == coordinateLocationX)
                    {
                        coordinateLocationX ++;
                    }
                    else if(coordinate.Item2 == coordinateLocationY)
                    {
                        coordinateLocationY ++;
                    }
                }
            }

            return new Tuple<int,int>(coordinateLocationX, coordinateLocationY);
        }
    }
}