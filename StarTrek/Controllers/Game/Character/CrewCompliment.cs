using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Controllers.Game.Character
{
    public class CrewCompliment : ICrewCompliment
    {
        public ICrewMember Captain { get; set; }
        public ICrewMember FirstOfficer { get; set; }
        public ICrewMember HeadOfEngineering { get; set; }
        public ICrewMember HeadOfSecurity { get; set; }
        public ICrewMember HeadOfMedical { get; set; }
        public ICrewMember HeadOfTactical { get; set; }
        public ICrewMember HeadOfScience { get;set; }
    }
}