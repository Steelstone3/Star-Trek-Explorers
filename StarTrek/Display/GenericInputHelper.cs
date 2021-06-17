using StarTrek.Contracts.Display;

namespace StarTrek.Display
{
    public class GenericInputHelper : IGenericInputHelper
    {
        public int GetNumericUserInput(IUserDisplay userDisplay, string message, int lowerBound, int upperBound)
        {
            string input;
            int validValue;

            do
            {
                input = userDisplay.GetUserInput(message);
            } while (!int.TryParse(input, out validValue) || validValue < lowerBound || validValue > upperBound);

            return validValue;
        }

        public double GetNumericUserInput(IUserDisplay userDisplay, string message, double lowerBound, double upperBound)
        {
            string input;
            double validValue;

            do
            {
                input = userDisplay.GetUserInput(message);
            } while (!double.TryParse(input, out validValue) || validValue < lowerBound || validValue > upperBound);

            return validValue;
        }

        public string GetStringUserInput(IUserDisplay userDisplay, string message)
        {
            return userDisplay.GetUserInput(message);
        }
    }
}