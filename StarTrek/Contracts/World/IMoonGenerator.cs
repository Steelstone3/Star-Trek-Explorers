namespace StarTrek.Contracts.World
{
    public interface IMoonGenerator
    {
        double GetDiameter(int id);
        double GetMass(int id);
        string GetName(int id);
    }
}