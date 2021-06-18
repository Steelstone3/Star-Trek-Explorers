using System.Collections.Generic;

namespace StarTrek.Contracts.Display
{
    public interface IGenericDisplayHelper
    {
        int GetNumericUserInput(string message, int lowerBound, int upperBound);
        double GetNumericUserInput(string message, double lowerBound, double upperBound);
        string GetStringUserInput(string message);
        string GetStringUserInput();
        void DisplayMessage(string message);
        void DisplayMenuItems(IEnumerable<string> menuItems);
    }
}