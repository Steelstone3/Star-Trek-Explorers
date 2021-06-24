using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;

namespace StarTrek.Controllers.Game.Character.Factories
{
    public class CharacterFactory : ICharacterFactory
    {
        public ICrewMember CreateCrewMember(ICrewRole crewRole, string name)
        {
            return new CrewMember(name, crewRole);
        }

        public ICrewCompliment AddCrewMemberToCrewCompliment(ICrewCompliment crewCompliment, ICrewMember crewMember)
        {
            switch (crewMember.CrewRole.Role)
            {
                case nameof(Captain):
                    crewCompliment.Captain = crewMember;
                    return crewCompliment;
                case nameof(FirstOfficer):
                    crewCompliment.FirstOfficer = crewMember;
                    return crewCompliment;
                case nameof(HeadOfEngineering):
                    crewCompliment.HeadOfEngineering = crewMember;
                    return crewCompliment;
                case nameof(HeadOfSecurity):
                    crewCompliment.HeadOfSecurity = crewMember;
                    return crewCompliment;
                case nameof(HeadOfMedical):
                    crewCompliment.HeadOfMedical = crewMember;
                    return crewCompliment;
                case nameof(HeadOfScience):
                    crewCompliment.HeadOfScience = crewMember;
                    return crewCompliment;
                case nameof(HeadOfTactical):
                    crewCompliment.HeadOfTactical = crewMember;
                    return crewCompliment;
                default:
                    return crewCompliment;
            }
        }
    }
}