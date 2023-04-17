using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Presenters
{
    public class PresenterShould
    {
        private readonly IPresenter presenter = new Presenter();

        [Fact]
        public void Construct()
        {
            // Then
            Assert.NotNull(presenter.ShipPresenter);
            Assert.NotNull(presenter.UniversePresenter);
        }
    }
}