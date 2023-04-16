namespace StarTrekExplorersTests.Entities
{
    public class Ship : IShip
    {
        public IIdentification Identification { get; } = new Identification();
    }
}