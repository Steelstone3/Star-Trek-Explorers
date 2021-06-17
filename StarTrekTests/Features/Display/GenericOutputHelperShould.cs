using System.Collections.Generic;
using Moq;
using StarTrek.Contracts.Display;
using StarTrek.Display;
using Xunit;

namespace StarTrekTests.Features
{
    public class GenericOutputHelperShould
    {
        [Theory]
        [InlineData("a user message")]
        [InlineData("grog is the best")]
        public void DisplayUserMessage(string message)
        {
            //A
            var userDisplayMock = new Mock<IUserDisplay>();
            userDisplayMock.Setup(x => x.DisplayMessage(message));
            var genericOutputHelper = new GenericOutputHelper(userDisplayMock.Object);

            //A
            genericOutputHelper.DisplayMessage(message);

            //A
            userDisplayMock.Verify(x => x.DisplayMessage(message));
        }

        [Fact]
        public void DisplayMenuItems()
        {
            //A
            var menuItems = CreateMenu();
            var userDisplayMock = new Mock<IUserDisplay>();
            userDisplayMock.Setup(x => x.DisplayMenuItems(menuItems));
            var genericOutputHelper = new GenericOutputHelper(userDisplayMock.Object);

            //A
            genericOutputHelper.DisplayMenuItems(menuItems);

            //A
            userDisplayMock.Verify(x => x.DisplayMenuItems(menuItems));
        }

        private List<string> CreateMenu()
        {
            return new List<string>() { "1. Grog is the best", "2. Grog is the besterer", "3. No Grogging is bestest" };
        }
    }
}