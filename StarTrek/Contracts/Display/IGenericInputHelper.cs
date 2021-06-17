namespace StarTrek.Contracts.Display
{
    public interface IGenericInputHelper
    {
        int GetNumericUserInput(IUserDisplay userDisplay, string message, int lowerBound, int upperBound);
        double GetNumericUserInput(IUserDisplay userDisplay, string message, double lowerBound, double upperBound);
        string GetStringUserInput(IUserDisplay userDisplay, string message);
    }
}