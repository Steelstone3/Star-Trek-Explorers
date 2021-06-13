using System;
using System.Collections.Generic;
using StarTrek.Contracts.World.CelestialBodies;

namespace StarTrek.Contracts.World.Builders
{
    public interface IStarSystemBuilder
    {
        string GetName(int id);
        string GetType(int id);
        double GetMass(int id);
        double GetDiameter(int id);
        Tuple<int, int> SetUniqueLocation(int coordinateLocationX, int coordinateLocationY, IEnumerable<IStarSystem> starSystems);
    }
}