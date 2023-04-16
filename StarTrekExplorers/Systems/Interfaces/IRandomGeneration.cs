namespace StarTrekExplorers.Systems.Interfaces
{
    public interface IRandomGeneration
    {
        int GetRandomInRange(int seed, int minimum, int maximum);
        int GetSeed();
    }
}