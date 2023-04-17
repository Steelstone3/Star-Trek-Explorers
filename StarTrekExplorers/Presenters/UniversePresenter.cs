using StarTrekExplorers.Presenters.Interfaces;

namespace StarTrekExplorers.Presenters
{
    internal class UniversePresenter : IUniversePresenter
    {
        private readonly Presenter presenter;

        public UniversePresenter(Presenter presenter)
        {
            this.presenter = presenter;
        }
    }
}