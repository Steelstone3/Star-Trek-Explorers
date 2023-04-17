using System.Collections.Generic;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Entities.Ships;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class ShipGeneration : IShipGeneration
    {
        public IEnumerable<IShip> GenerateShips(IPresenter presenter, Faction faction)
        {
            List<IShip> stars = new();

            AddShips(presenter, faction, stars);

            return stars;
        }

        private static void AddShips(IPresenter presenter, Faction faction, List<IShip> stars)
        {
            RandomGeneration rng = new();
            int amount = rng.GetRandomInRange(rng.GetSeed(), 1, 10);

            for (int i = 0; i < amount; i++)
            {
                IShip star = CreateShip(presenter, rng, faction);
                stars.Add(star);
            }
        }

        private static Ship CreateShip(IPresenter presenter, RandomGeneration rng, Faction faction)
        {
            return new Ship(presenter, rng.GetSeed(), faction);
        }
    }
}