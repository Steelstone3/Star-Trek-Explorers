namespace StarTrekExplorersTests.Systems
{
    public interface IRandomGeneration
    {
        int GetRandomInRange(int seed, int minimum, int maximum);
        int GetSeed();
    }
}