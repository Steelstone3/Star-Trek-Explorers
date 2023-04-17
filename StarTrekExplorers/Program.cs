using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers
{
    internal static class Program
    {
        internal static void Main()
        {
            IPresenter presenter = new Presenter();
            IGame game = new Game(presenter);
            presenter.ShipPresenter.PrintShipName(game.PlayerShip);
            presenter.ShipPresenter.PrintShipNames(game.FederationShips);
            presenter.ShipPresenter.PrintShipNames(game.KlingonShips);
            presenter.UniversePresenter.PrintStars(game.Universe.Stars);
        }
    }
}
