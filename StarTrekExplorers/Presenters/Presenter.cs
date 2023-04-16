using System;
using System.Collections.Generic;
using Spectre.Console;

namespace StarTrekExplorers.Presenters
{
    public class Presenter : IPresenter
    {
        public Presenter()
        {
            ShipPresenter = new ShipPresenter(this);
        }

        public IShipPresenter ShipPresenter { get; }

        public void Print(string message)
        {
            AnsiConsole.WriteLine(message);
        }

        public string SelectString(string message, List<string> options)
        {
            return AnsiConsole.Prompt(new SelectionPrompt<string>()
                .Title(message)
                .AddChoices(options));
        }
    }
}