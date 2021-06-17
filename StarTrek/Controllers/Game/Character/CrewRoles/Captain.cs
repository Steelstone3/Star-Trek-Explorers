namespace StarTrek.Controllers.Game.Character.CrewRoles
{
    public class Captain : CrewRole
    {
        public Captain()
        {
            Rank = "Captain";
            Role = nameof(Captain);
        }
    }
}