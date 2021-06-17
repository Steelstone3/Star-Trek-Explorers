using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Contracts.Character
{
    public interface ICrewCompliment
    {
        ICrewMember Captain { get; set; }
        ICrewMember FirstOfficer { get; set; }
    }
}