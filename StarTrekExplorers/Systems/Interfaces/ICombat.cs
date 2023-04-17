using StarTrekExplorers.Entities.Interfaces;

namespace StarTrekExplorers.Systems.Interfaces
{
    public interface ICombat
    {
        void Start(int seed, IShip playerShip, IShip hostileShip);
    }
}