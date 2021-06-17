using StarTrek.Contracts.Display;

namespace StarTrek.Display
{
    public class GenericInputHelper : IGenericInputHelper
    {
        private IUserDisplay _userDisplay;

        public GenericInputHelper(IUserDisplay userDisplay)
        {
            _userDisplay = userDisplay;
        }

        public int GetNumericUserInput(string message, int lowerBound, int upperBound)
        {
            string input;
            int validValue;

            do
            {
                input = _userDisplay.GetUserInput(message);
            } while (!int.TryParse(input, out validValue) || validValue < lowerBound || validValue > upperBound);

            return validValue;
        }

        public double GetNumericUserInput(string message, double lowerBound, double upperBound)
        {
            string input;
            double validValue;

            do
            {
                input = _userDisplay.GetUserInput(message);
            } while (!double.TryParse(input, out validValue) || validValue < lowerBound || validValue > upperBound);

            return validValue;
        }

        public string GetStringUserInput(string message)
        {
            return _userDisplay.GetUserInput(message);
        }

        public string GetStringUserInput()
        {
            return _userDisplay.GetUserInput();
        }
    }
}