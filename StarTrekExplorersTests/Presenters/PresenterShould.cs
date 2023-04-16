using StarTrekExplorers.Presenters;
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
        }
    }
}