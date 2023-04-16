using StarTrekExplorers.Components.Interfaces;

namespace StarTrekExplorersTests.Entities
{
    public class Hull : IDefense
    {
        public int Current { get; private set; } = 100;
        public int Maximum { get; } = 100;
        public int RepairRate { get; } = 5;

        public void TakeDamage(int damage)
        {
            if (damage >= Current)
            {
                Current = 0;
            }
            else
            {
                Current -= damage;
            }
        }
    }
}