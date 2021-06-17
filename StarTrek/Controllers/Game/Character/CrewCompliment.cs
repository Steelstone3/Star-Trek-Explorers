using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Controllers.Game.Character
{
    public class CrewCompliment : ICrewCompliment
    {
        public ICrewMember Captain { get;set; }
        public ICrewMember FirstOfficer { get; set; }
    }
}