using System;

namespace StarTrekExplorers.Components.Ship.Names
{
    public interface IShipNames
    {
        string GetShipName(int seed, Faction faction);
    }
}