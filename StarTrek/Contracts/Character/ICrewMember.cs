using StarTrek.Contracts.Character;

namespace StarTrek.Controllers.Game.Character.Factories
{
    public interface ICrewMember
    {
        ICrewRole CrewRole{get;}
        string Name{get;}
    }
}