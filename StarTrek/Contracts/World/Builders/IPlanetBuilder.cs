namespace StarTrek.Contracts.World.Builders
{
    public interface IPlanetBuilder
    {
        string GetName(int id);
        string GetAtmoshere(int id);
        double GetMass(int id);
        double GetDiameter(int id);
    }
}