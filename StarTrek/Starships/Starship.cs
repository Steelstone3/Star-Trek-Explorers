using StarTrek.Contracts.Starships;
using StarTrek.Contracts.Starships.Builders;

namespace StarTrek.Starships
{
    public class Starship : IStarship
    {
        public Starship(IStarshipBuilder starshipBuilder)
        {
            starshipBuilder.BuildStarship();
        }

        public int CoordinateLocationX { get; set; }
        public int CoordinateLocationY { get; set; }
    }
}