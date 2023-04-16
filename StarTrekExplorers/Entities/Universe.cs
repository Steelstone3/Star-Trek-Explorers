using System.Collections.Generic;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Universe : IUniverse
    {
        public IEnumerable<IStar> Stars { get; } = new StarGeneration().GenerateStars();
    }
}