namespace StarTrek.Contracts.Display
{
    public interface IGenericInputHelper
    {
        int GetNumericUserInput(string message, int lowerBound, int upperBound);
        double GetNumericUserInput(string message, double lowerBound, double upperBound);
        string GetStringUserInput(string message);
        string GetStringUserInput();
    }
}