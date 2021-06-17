using Moq;
using StarTrek.Contracts.Display;
using StarTrek.Display;
using Xunit;

namespace StarTrekTests.Features
{
    public class GenericInputHelperShould
    {
        [Theory]
        [InlineData("Select Character", 1, 5, "4", 4)]
        [InlineData("Enter Warp Factor", 3, 8, "5", 5)]
        [InlineData("Bob", 5, 10, "6", 6)]
        public void GetUsersNumericIntergerInput(string message, int lowerBound, int upperBound, string fakeInput, int expectedOutput)
        {
            var userDisplayMock = CreateUserDisplayMock(message, fakeInput);
            var genericInput = new GenericInputHelper();

            var validUserValue = genericInput.GetNumericUserInput(userDisplayMock.Object, message, lowerBound, upperBound);

            Assert.InRange(validUserValue, lowerBound, upperBound);
            Assert.Equal(expectedOutput, validUserValue);
            userDisplayMock.Verify(x => x.GetUserInput(message));
        }

        [Theory]
        [InlineData("Select Character", 1, 5, "4", 4)]
        [InlineData("Enter Warp Factor", 3.2, 8.5, "5.2", 5.2)]
        [InlineData("Bob", 5.5, 10.3, "6.7", 6.7)]
        public void GetUsersNumericDoubleInput(string message, double lowerBound, double upperBound, string fakeInput, double expectedOutput)
        {
            var userDisplayMock = CreateUserDisplayMock(message, fakeInput);
            var genericInput = new GenericInputHelper();

            var validUserValue = genericInput.GetNumericUserInput(userDisplayMock.Object, message, lowerBound, upperBound);

            Assert.InRange(validUserValue, lowerBound, upperBound);
            Assert.Equal(expectedOutput, validUserValue);
            userDisplayMock.Verify(x => x.GetUserInput(message));
        }

        [Theory]
        [InlineData("What did you do today", "Went to the shops")]
        public void GetUsersStringInput(string message, string fakeInput)
        {
            var userDisplayMock = CreateUserDisplayMock(message, fakeInput);
            var genericInput = new GenericInputHelper();

            var validUserValue = genericInput.GetStringUserInput(userDisplayMock.Object, message);

            Assert.NotNull(validUserValue);
            Assert.NotEmpty(validUserValue);
            Assert.Equal(fakeInput, validUserValue);
            userDisplayMock.Verify(x => x.GetUserInput(message));
        }

        private Mock<IUserDisplay> CreateUserDisplayMock(string message, string fakeInput)
        {
            var userDisplayMock = new Mock<IUserDisplay>();
            userDisplayMock.Setup(x => x.GetUserInput(message)).Returns(fakeInput);

            return userDisplayMock;
        }
    }
}