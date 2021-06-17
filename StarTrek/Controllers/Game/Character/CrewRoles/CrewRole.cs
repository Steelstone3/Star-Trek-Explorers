using StarTrek.Contracts.Character;

namespace StarTrek.Controllers.Game.Character.CrewRoles
{
    public abstract class CrewRole : ICrewRole
    {
        /*public enum ShipRank
        {
            Captain,
            FirstOfficer,
            Commander,
            LieutenantCommander,
            Lieutenant,
            Ensign,
            Cadet,
        }*/

        public string Rank { get; protected set; }
        public string Role { get; protected set; }
    }
}