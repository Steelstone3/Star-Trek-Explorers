using System.Collections.Generic;
using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Contracts.Character
{
    public interface ICrewCompliment
    {
        ICrewMember Captain { get; set; }
        ICrewMember FirstOfficer { get; set; }
        ICrewMember HeadOfEngineering { get; set;}
        ICrewMember HeadOfSecurity { get; set; }
        ICrewMember HeadOfMedical { get; set; }
        ICrewMember HeadOfTactical { get; set; }
        ICrewMember HeadOfScience { get; set; }
    }
}