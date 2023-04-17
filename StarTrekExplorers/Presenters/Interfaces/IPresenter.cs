using System.Collections.Generic;

namespace StarTrekExplorers.Presenters.Interfaces
{
    public interface IPresenter
    {
        void NewLine();
        void Print(string message);
        string SelectString(string message, List<string> options);
        IShipPresenter ShipPresenter { get; }
        IUniversePresenter UniversePresenter { get; }
    }
}