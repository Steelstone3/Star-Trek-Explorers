using System.Collections.Generic;

namespace StarTrek.Contracts.Display
{
    public interface IGenericOutputHelper
    {
        void DisplayMessage(string message);
        void DisplayMenuItems(IEnumerable<string> menuItems);
    }
}