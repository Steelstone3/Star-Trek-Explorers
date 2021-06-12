namespace StarTrek.Contracts.World.Builders
{
    public interface IStarSystemBuilder
    {
        string GetName(int id);
        string GetType(int id);
        double GetMass(int id);
        double GetDiameter(int id);
    }
}