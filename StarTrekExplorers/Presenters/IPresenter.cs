using System;
using System.Collections.Generic;

namespace StarTrekExplorers.Presenters
{
    public interface IPresenter
    {
        void Print(string message);
        string SelectString(string message, List<string> options);
        IShipPresenter ShipPresenter { get; }
    }
}