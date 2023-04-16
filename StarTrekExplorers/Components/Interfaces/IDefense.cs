using System;

namespace StarTrekExplorers.Components.Interfaces
{
    public interface IDefense : IDamageTaker
    {
        int Current { get; }
        int Maximum { get; }
        int RepairRate { get; }
    }
}