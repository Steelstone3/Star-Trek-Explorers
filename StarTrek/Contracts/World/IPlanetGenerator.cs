namespace StarTrek.Contracts
{
    public interface IPlanetGenerator
    {   
        string GetName(int id);
        string GetAtmoshere(int id);
        double GetMass(int id);
        double GetDiameter(int id);
    }
}