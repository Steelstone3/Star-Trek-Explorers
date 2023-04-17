using System.Collections.Generic;
using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Systems.Interfaces;
using StarTrekExplorersTests.Entities;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorers.Systems
{
    public class ShipGeneration : IShipGeneration
    {
        public IEnumerable<IShip> GenerateShips(Faction faction)
        {
            List<IShip> stars = new();

            RandomGeneration rng = new();
            int amount = rng.GetRandomInRange(rng.GetSeed(), 1, 10);
            AddStars(rng, faction, stars, amount);

            return stars;
        }

        private static void AddStars(RandomGeneration rng, Faction faction, List<IShip> stars, int amount)
        {
            for (int i = 0; i < amount; i++)
            {
                IShip star = CreateShip(rng, faction);
                stars.Add(star);
            }
        }

        private static Ship CreateShip(RandomGeneration rng, Faction faction)
        {
            return new Ship(new Presenter(), rng.GetSeed(), faction);
        }
    }
}