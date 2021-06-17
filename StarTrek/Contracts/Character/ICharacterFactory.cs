using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Contracts.Character
{
    public interface ICharacterFactory
    {
        ICrewMember CreateCrewMember(ICrewRole crewRole, string name);
        ICrewCompliment AddCrewMemberToCrewCompliment(ICrewCompliment crewCompliment, ICrewMember crewMember);
    }
}