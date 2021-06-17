using System.Collections.Generic;

namespace StarTrek.Contracts.Display
{
    public interface IUserDisplay
    {
        /*void DisplayMessage(string message);
        void DisplayMenuItems(IEnumerable<string> message);
        string GetUserInput();*/
        string GetUserInput(string message);
    }
}