using StarTrek.Contracts.Starships;
using StarTrek.Contracts.Starships.Builders;
using StarTrek.Contracts.World;

namespace StarTrek.Starships
{
    public class Starship : IStarship, ILocation
    {
        public Starship(IStarshipBuilder starshipBuilder)
        {
            starshipBuilder.BuildStarship();
        }

        public int CoordinateLocationX { get; set; }
        public int CoordinateLocationY { get; set; }
    }
}