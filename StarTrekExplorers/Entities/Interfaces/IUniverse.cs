using System.Collections.Generic;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Entities.Interfaces
{
    public interface IUniverse
    {
        IEnumerable<IStar> Stars { get; }
    }
}