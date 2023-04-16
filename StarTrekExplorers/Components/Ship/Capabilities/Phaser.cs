using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorersTests.Systems;

namespace StarTrekExplorersTests.Entities
{
    public class Phaser : IWeapon
    {
        public string Name => "Phaser";
        public int Maximum => 10;
        public int Minimum => 5;

        public int DealDamage(int seed)
        {
            RandomGeneration randomGeneration = new();
            return randomGeneration.GetRandomInRange(seed, Minimum, Maximum);
        }
    }
}