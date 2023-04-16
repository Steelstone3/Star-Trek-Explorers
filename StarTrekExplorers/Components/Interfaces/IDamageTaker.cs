namespace StarTrekExplorers.Components.Interfaces
{
    public interface IDamageTaker
    {
        int Current { get; }
        int Maximum { get; }
        int RepairRate { get; }
        void TakeDamage(int damage);
    }
}