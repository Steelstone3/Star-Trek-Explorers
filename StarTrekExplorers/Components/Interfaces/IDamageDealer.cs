namespace StarTrekExplorers.Components.Interfaces
{
    public interface IDamageDealer
    {
        int Maximum { get; }
        int Minimum { get; }
        int DealDamage(int seed);
    }
}