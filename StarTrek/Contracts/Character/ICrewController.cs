using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Contracts.Character
{
    public interface ICrewController
    {
        ICrewCompliment CrewCompliment { get; set; }
        void AddCrewMember(ICrewRole crewRole, string name);
        void AddCrewMember(ICrewMember crewMember);
    }
}