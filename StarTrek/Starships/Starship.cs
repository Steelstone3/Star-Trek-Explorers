using StarTrek.Contracts.Starships;
using StarTrek.Contracts.World;

namespace StarTrek.Starships
{
    public class Starship : IStarship, ILocation
    {
        public Starship()
        {

        }
        
        public Starship(int coordinateLocationX, int coordinateLocationY)
        {
            CoordinateLocationX = coordinateLocationX;
            CoordinateLocationY = coordinateLocationY;
        }

        public int CoordinateLocationX { get; set; }
        public int CoordinateLocationY { get; set; }
    }
}