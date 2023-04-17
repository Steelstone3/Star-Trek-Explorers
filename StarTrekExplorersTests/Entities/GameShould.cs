using Moq;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters;
using Xunit;

namespace StarTrekExplorersTests.Entities
{
    public class GameShould
    {
        readonly Mock<IPresenter> presenter = new();
        private readonly IGame game;
        public GameShould()
        {
            game = new Game(presenter.Object);
        }

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(game.PlayerShip);
            Assert.NotNull(game.Universe);
            Assert.NotEmpty(game.FederationShips);
            Assert.NotEmpty(game.KlingonShips);
        }
    }
}