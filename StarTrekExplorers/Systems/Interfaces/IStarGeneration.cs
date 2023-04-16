using System.Collections.Generic;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface IStarGeneration
    {
        IEnumerable<IStar> GenerateStars();
    }
}