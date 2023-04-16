using System;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IWeapon : IDamageDealer
    {
        int Maximum { get; }
        int Minimum { get; }
    }
}