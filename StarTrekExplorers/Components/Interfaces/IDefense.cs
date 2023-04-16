namespace StarTrekExplorers.Components.Interfaces
{
    public interface IDefense
    {
        int Current { get; }
        int Maximum { get; }
        int RepairRate { get; }
        void TakeDamage(int damage);
    }
}