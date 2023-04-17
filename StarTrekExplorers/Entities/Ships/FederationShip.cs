using StarTrekExplorers.Components.Ship.Names;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Entities.Ships
{
    public class FederationShip : Ship
    {
        public FederationShip(IPresenter presenter, int seed, Faction faction) : base(presenter, seed, faction)
        {
        }
    }
}