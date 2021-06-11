namespace StarTrek.Contracts.World
{
    public interface IStarSystemGenerator
    {
        string GetName(int id);
        string GetType(int id);
        double GetMass(int id);
        double GetDiameter(int id);
    }
}