namespace StarTrek.Contracts.World.Builders
{
    public interface IMoonBuilder
    {
        double GetDiameter(int id);
        double GetMass(int id);
        string GetName(int id);
    }
}