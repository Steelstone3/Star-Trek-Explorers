using System.Linq;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorers.Systems;
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
            presenter.NewLine();
            presenter.ShipPresenter.PrintShipNames(game.FederationShips);
            presenter.NewLine();
            presenter.ShipPresenter.PrintShipNames(game.KlingonShips);
            presenter.UniversePresenter.PrintStars(game.Universe.Stars);

            new Combat(presenter).Start(0, game.PlayerShip, game.KlingonShips.ToArray()[0]);
        }
    }
}
