using System;
using System.Collections.Generic;
using StarTrek.Contracts.Display;

namespace StarTrek.Display
{
    public class UserDisplay : IUserDisplay
    {
        /*public void DisplayMessage(string message)
        {
            Console.WriteLine(message);
        }

        public void DisplayMenuItems(IEnumerable<string> menuItems)
        {
            foreach (var item in menuItems)
            {
                Console.WriteLine(item);
            }
        }

        public string GetUserInput()
        {
            return Console.ReadLine();
        }*/

        public string GetUserInput(string message)
        {
            Console.WriteLine(message);
            return Console.ReadLine();
        }
    }
}