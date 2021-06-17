using System.Collections.Generic;
using StarTrek.Contracts.Display;

namespace StarTrek.Display
{
    public class GenericOutputHelper : IGenericOutputHelper
    {
        private IUserDisplay _userDisplay;

        public GenericOutputHelper(IUserDisplay userDisplay)
        {
            _userDisplay = userDisplay;
        }

        public void DisplayMessage(string message)
        {
            _userDisplay.DisplayMessage(message);
        }

        public void DisplayMenuItems(IEnumerable<string> menuItems)
        {
            _userDisplay.DisplayMenuItems(menuItems);
        }
    }
}