using StarTrek.Contracts.Character;

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
                case "Captain":
                    crewCompliment.Captain = crewMember;
                    return crewCompliment;
                default:
                    return crewCompliment;
            }
        }
    }
}