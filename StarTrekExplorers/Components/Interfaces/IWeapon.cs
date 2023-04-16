using System;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IWeapon : IDamageDealer
    {
        string Name { get; }
        int Maximum { get; }
        int Minimum { get; }
    }
}