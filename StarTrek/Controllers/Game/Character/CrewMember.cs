using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Controllers.Game.Character
{
    public class CrewMember : ICrewMember
    {
        public CrewMember(string name, ICrewRole crewRole)
        {
            Name = name;
            CrewRole = crewRole;
        }

        public ICrewRole CrewRole { get; private set; }

        public string Name { get; private set; }
    }
}